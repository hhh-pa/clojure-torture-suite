//! Seeded random Clojure program generator (Csmith-style).
//!
//! Emits self-contained, deterministic torture programs in two modes:
//!
//! - `interp`: a broad safe subset (arithmetic, strings, vectors, maps,
//!   sets, seqs, `let`/`if`/`do`/`loop`, helper `defn`s, `prn`).
//! - `aot`: the `clojure-rs build` pure subset only (top-level fixed-arity
//!   `defn`, `if`/`do`/`let`, direct self-recursion, the 13 primitives
//!   `+ - * quot < <= > >= = count nth get conj`, and `print`).
//!
//! # Safety contract (zero false positives)
//!
//! Every generated program must exit 0 with byte-identical stdout on the
//! JVM oracle and on `clojure-rs`. The generator enforces this by
//! construction:
//!
//! - type-directed: `=` only joins identical scalar types (never the
//!   `/`-on-ints ratio, never cross-type numerics);
//! - `quot`/`rem`/`mod` divisors are nonzero literals; `*` operands are
//!   small; every int expr carries a `[lo, hi]` range and anything
//!   escaping ±1e6 falls back to a literal (no overflow, no div-by-zero);
//! - `nth`/`get`/`assoc`/`subvec` indices are literal and in range
//!   (2-arity `get` only on vectors — it returns nil on seqs);
//!   sequential/map/set lengths are tracked (`None` = unknown, and
//!   unknown lengths only feed total operations like `count`/`conj`);
//! - `subs` only touches ASCII strings (indices are UTF-16 units);
//! - maps and sets are never printed (order-free) and never sequenced;
//! - no floats, no ratios, no randomness, no state, no I/O besides the
//!   final `prn`/`print`, no `loop` in AOT mode, no mutual recursion;
//! - `loop`/`recur` bounds are literals, and self-recursive helpers are
//!   only called with small literals (termination guaranteed).
//!
//! Determinism: the only entropy is a xorshift64* stream seeded by
//! `--seed`; no `HashMap` iteration anywhere on the emission path.

use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct GenArgs {
    /// Program shape: interp or aot
    #[arg(long, default_value = "interp")]
    pub mode: String,
    /// First seed to emit
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    /// How many programs to emit
    #[arg(long, default_value_t = 1)]
    pub count: u64,
    /// Max expression nesting depth
    #[arg(long, default_value_t = 4)]
    pub depth: u32,
    /// Write programs to DIR as seed-N.clj (default: stdout, count must be 1)
    #[arg(long)]
    pub out: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenMode {
    Interp,
    Aot,
}

impl GenMode {
    fn parse(s: &str) -> Result<GenMode, String> {
        match s {
            "interp" => Ok(GenMode::Interp),
            "aot" => Ok(GenMode::Aot),
            other => Err(format!("bad --mode '{other}' (want interp or aot)")),
        }
    }
}

// ---------------------------------------------------------------- RNG ---

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64* (deterministic across platforms).
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: u64) -> u64 {
        debug_assert!(n > 0);
        self.next() % n
    }
    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        debug_assert!(lo < hi);
        lo + (self.next() % ((hi - lo) as u64)) as i64
    }
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.below(xs.len() as u64) as usize]
    }
    fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
}

// --------------------------------------------------------------- AST ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ty {
    Int,
    Bool,
    Str,
    /// Vector of ints (elements always int; nesting only at result level).
    Vec,
    /// Sequential non-vector of ints (range/map/filter/take/... results).
    Seq,
    /// Map with keyword keys and int values (never printed/sequenced).
    Map,
    /// Set of ints (never printed/sequenced).
    Set,
}

/// A generated expression with its type, known length (for sequential,
/// string, map, set values; `None` = unknown), and int range.
#[derive(Debug, Clone)]
struct Typed {
    ty: Ty,
    code: String,
    len: Option<usize>,
    lo: i64,
    hi: i64,
}

impl Typed {
    fn leaf(ty: Ty, code: String) -> Typed {
        Typed {
            ty,
            code,
            len: None,
            lo: 0,
            hi: 0,
        }
    }
    fn int(code: String, lo: i64, hi: i64) -> Typed {
        Typed {
            ty: Ty::Int,
            code,
            len: None,
            lo,
            hi,
        }
    }
}

#[derive(Debug, Clone)]
struct Var {
    name: String,
    ty: Ty,
    len: Option<usize>,
    lo: i64,
    hi: i64,
}

#[derive(Debug, Clone)]
struct FnParam {
    name: String,
    ty: Ty,
}

#[derive(Debug, Clone)]
struct FnDef {
    name: String,
    params: Vec<FnParam>,
    ret: Ty,
    body: String,
    /// Self-recursive fns are only ever called with small literals at
    /// their dedicated call site; never via gen_call (termination).
    recursive: bool,
}

struct Ctx {
    rng: Rng,
    mode: GenMode,
    max_depth: u32,
    vars: Vec<Var>,
    fns: Vec<FnDef>,
    var_counter: u32,
    fn_counter: u32,
    fuel: u32,
}

impl Ctx {
    fn spend(&mut self) -> bool {
        if self.fuel == 0 {
            return false;
        }
        self.fuel -= 1;
        true
    }
    fn fresh_var(&mut self) -> String {
        let n = self.var_counter;
        self.var_counter += 1;
        format!("v{n}")
    }
    fn vars_of(&self, ty: Ty) -> Vec<Var> {
        self.vars.iter().filter(|v| v.ty == ty).cloned().collect()
    }
}

const RANGE_CAP: i64 = 1_000_000;

fn in_range(lo: i64, hi: i64) -> bool {
    lo >= -RANGE_CAP && hi <= RANGE_CAP
}

fn escape_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

const STR_POOL: &[&str] = &["a", "bc", "hello", "x y", "s.1", "q", "xy"];
const STR_POOL_WIDE: &[&str] = &["a", "bc", "hello", "x y", "s.1", "q", "xy", "λ"];
const KW_POOL: &[&str] = &["a", "b", "c", "d", "e", "f", "g", "h"];

// --------------------------------------------------------- generators ---

fn gen_int(ctx: &mut Ctx, depth: u32) -> Typed {
    if depth == 0 || !ctx.spend() {
        return int_literal(ctx);
    }
    if ctx.rng.chance(4) {
        return gen_do(ctx, depth, Ty::Int);
    }
    // Collect candidate shapes; weights in comments.
    let roll = ctx.rng.below(100);
    if roll < 22 {
        int_literal(ctx) // 22
    } else if roll < 34 {
        let vs = ctx.vars_of(Ty::Int);
        if vs.is_empty() {
            int_literal(ctx)
        } else {
            let v = ctx.rng.pick(&vs).clone();
            Typed::int(v.name, v.lo, v.hi)
        }
    } else if roll < 46 {
        // (+ a b) / (- a b)
        let a = gen_int(ctx, depth - 1);
        let b = gen_int(ctx, depth - 1);
        let op = if ctx.rng.chance(50) { "+" } else { "-" };
        let (lo, hi) = if op == "+" {
            (a.lo + b.lo, a.hi + b.hi)
        } else {
            (a.lo - b.hi, a.hi - b.lo)
        };
        if !in_range(lo, hi) {
            return int_literal(ctx);
        }
        Typed::int(format!("({op} {} {})", a.code, b.code), lo, hi)
    } else if roll < 54 {
        // (* a b) with small operands.
        let a = gen_small_int(ctx, depth - 1);
        let b = gen_small_int(ctx, depth - 1);
        let cands = [a.lo * b.lo, a.lo * b.hi, a.hi * b.lo, a.hi * b.hi];
        let (lo, hi) = (
            cands.iter().min().copied().unwrap(),
            cands.iter().max().copied().unwrap(),
        );
        if !in_range(lo, hi) {
            return int_literal(ctx);
        }
        Typed::int(format!("(* {} {})", a.code, b.code), lo, hi)
    } else if roll < 62 {
        // (quot/rem/mod a LIT) with nonzero literal divisor
        // (AOT subset has quot only).
        let a = gen_int(ctx, depth - 1);
        let op = if ctx.mode == GenMode::Aot {
            "quot"
        } else {
            *ctx.rng.pick(&["quot", "rem", "mod"])
        };
        let d = ctx.rng.range(1, 10);
        let (lo, hi) = match op {
            "quot" => {
                let (l, h) = (a.lo / d, a.hi / d);
                (l.min(h), l.max(h))
            }
            _ => (-(d - 1), d - 1),
        };
        Typed::int(format!("({op} {} {d})", a.code), lo, hi)
    } else if roll < 68 {
        // (count coll)
        let c = gen_sized(ctx, depth - 1);
        let (lo, hi) = match c.len {
            Some(n) => (n as i64, n as i64),
            None => (0, 8),
        };
        Typed::int(format!("(count {})", c.code), lo, hi)
    } else if roll < 74 {
        // (nth/get v i) on known-length vectors (seqs in interp too).
        gen_nth(ctx, depth)
    } else if roll < 80 {
        // (if b a c)
        let b = gen_bool(ctx, depth - 1);
        let a = gen_int(ctx, depth - 1);
        let c = gen_int(ctx, depth - 1);
        Typed::int(
            format!("(if {} {} {})", b.code, a.code, c.code),
            a.lo.min(c.lo),
            a.hi.max(c.hi),
        )
    } else if roll < 86 && ctx.mode == GenMode::Interp {
        // (inc/dec/abs/min/max ...) — not AOT primitives.
        let a = gen_int(ctx, depth - 1);
        match ctx.rng.below(5) {
            0 => Typed::int(format!("(inc {})", a.code), a.lo + 1, a.hi + 1),
            1 => Typed::int(format!("(dec {})", a.code), a.lo - 1, a.hi - 1),
            2 => {
                let hi = a.lo.abs().max(a.hi.abs());
                Typed::int(format!("(abs {})", a.code), 0, hi)
            }
            _ => {
                let b = gen_int(ctx, depth - 1);
                let op = if ctx.rng.chance(50) { "min" } else { "max" };
                Typed::int(
                    format!("({op} {} {})", a.code, b.code),
                    a.lo.min(b.lo),
                    a.hi.max(b.hi),
                )
            }
        }
    } else if roll < 90 {
        gen_let(ctx, depth, Ty::Int)
    } else if roll < 94 && ctx.mode == GenMode::Interp {
        gen_loop(ctx, depth)
    } else if roll < 97 {
        // call an int-returning fn, else literal
        gen_call(ctx, depth, Ty::Int).unwrap_or_else(|| int_literal(ctx))
    } else if ctx.mode == GenMode::Interp {
        // (get m k default): total, int-typed (map values are small).
        let m = gen_map(ctx, depth - 1);
        let k = format!(":{}", ctx.rng.pick(KW_POOL));
        let d = gen_int(ctx, depth - 1);
        Typed::int(
            format!("(get {} {k} {})", m.code, d.code),
            d.lo.min(-99),
            d.hi.max(99),
        )
    } else {
        // AOT: (get v i default) over vectors (total, any index).
        let v = gen_vec(ctx, depth - 1);
        let i = ctx.rng.range(0, 10);
        let d = gen_int(ctx, depth - 1);
        Typed::int(format!("(get {} {i} {})", v.code, d.code), d.lo, d.hi)
    }
}

fn int_literal(ctx: &mut Ctx) -> Typed {
    let v = ctx.rng.range(-50, 200);
    Typed::int(v.to_string(), v, v)
}

/// Small ints (|v| <= 9) for `*` operands.
fn gen_small_int(ctx: &mut Ctx, depth: u32) -> Typed {
    if depth == 0 || !ctx.spend() {
        let v = ctx.rng.range(-9, 10);
        return Typed::int(v.to_string(), v, v);
    }
    let small_vars: Vec<Var> = ctx
        .vars
        .iter()
        .filter(|v| v.ty == Ty::Int && v.lo >= -9 && v.hi <= 9)
        .cloned()
        .collect();
    let roll = ctx.rng.below(100);
    if roll < 60 || small_vars.is_empty() {
        let v = ctx.rng.range(-9, 10);
        Typed::int(v.to_string(), v, v)
    } else if roll < 80 {
        let v = ctx.rng.pick(&small_vars).clone();
        Typed::int(v.name, v.lo, v.hi)
    } else {
        let a = ctx.rng.range(-4, 5);
        let b = ctx.rng.range(-4, 5);
        let op = if ctx.rng.chance(50) { "+" } else { "-" };
        let (lo, hi) = if op == "+" {
            (a + b, a + b)
        } else {
            (a - b, a - b)
        };
        Typed::int(format!("({op} {a} {b})"), lo.min(hi), lo.max(hi))
    }
}

fn gen_nth(ctx: &mut Ctx, depth: u32) -> Typed {
    // Known-length int sequential + literal in-range index
    // (AOT: vectors only — seqs are out of subset).
    for _ in 0..8 {
        let v = if ctx.mode == GenMode::Aot || ctx.rng.chance(60) {
            gen_vec(ctx, depth - 1)
        } else {
            gen_seq(ctx, depth - 1)
        };
        if let Some(n) = v.len {
            if n > 0 {
                let i = ctx.rng.below(n as u64) as usize;
                // 2-arity get returns nil on seqs (not ILookup): nth there.
                let op = if v.ty == Ty::Vec && ctx.rng.chance(50) {
                    "get"
                } else {
                    "nth"
                };
                return Typed::int(format!("({op} {} {i})", v.code), -200, 200);
            }
        }
    }
    int_literal(ctx)
}

fn gen_bool(ctx: &mut Ctx, depth: u32) -> Typed {
    if depth == 0 || !ctx.spend() {
        return Typed::leaf(Ty::Bool, ctx.rng.pick(&["true", "false"]).to_string());
    }
    if ctx.rng.chance(4) {
        return gen_do(ctx, depth, Ty::Bool);
    }
    let roll = ctx.rng.below(100);
    if roll < 12 {
        Typed::leaf(Ty::Bool, ctx.rng.pick(&["true", "false"]).to_string())
    } else if roll < 20 {
        let vs = ctx.vars_of(Ty::Bool);
        if vs.is_empty() {
            Typed::leaf(Ty::Bool, "true".into())
        } else {
            Typed::leaf(Ty::Bool, ctx.rng.pick(&vs).name.clone())
        }
    } else if roll < 44 {
        // comparisons over ints (same type only)
        let a = gen_int(ctx, depth - 1);
        let b = gen_int(ctx, depth - 1);
        let op = *ctx.rng.pick(&["<", "<=", ">", ">=", "="]);
        Typed::leaf(Ty::Bool, format!("({op} {} {})", a.code, b.code))
    } else if roll < 52 {
        // = over bools or strings (same type only)
        if ctx.rng.chance(50) {
            let a = gen_bool(ctx, depth - 1);
            let b = gen_bool(ctx, depth - 1);
            Typed::leaf(Ty::Bool, format!("(= {} {})", a.code, b.code))
        } else {
            let a = gen_str(ctx, depth - 1);
            let b = gen_str(ctx, depth - 1);
            Typed::leaf(Ty::Bool, format!("(= {} {})", a.code, b.code))
        }
    } else if roll < 64 && ctx.mode == GenMode::Interp {
        // and/or/not and the numeric predicates are not AOT primitives.
        let a = gen_bool(ctx, depth - 1);
        let b = gen_bool(ctx, depth - 1);
        let op = *ctx.rng.pick(&["and", "or"]);
        Typed::leaf(Ty::Bool, format!("({op} {a} {b})", a = a.code, b = b.code))
    } else if roll < 70 && ctx.mode == GenMode::Interp {
        let a = gen_bool(ctx, depth - 1);
        Typed::leaf(Ty::Bool, format!("(not {})", a.code))
    } else if roll < 78 && ctx.mode == GenMode::Interp {
        let a = gen_int(ctx, depth - 1);
        let p = *ctx.rng.pick(&["zero?", "pos?", "neg?", "even?", "odd?"]);
        Typed::leaf(Ty::Bool, format!("({p} {})", a.code))
    } else if roll < 84 && ctx.mode == GenMode::Interp {
        let m = gen_map(ctx, depth - 1);
        let k = format!(":{}", ctx.rng.pick(KW_POOL));
        Typed::leaf(Ty::Bool, format!("(contains? {} {k})", m.code))
    } else if roll < 88 && ctx.mode == GenMode::Interp {
        let c = gen_sized(ctx, depth - 1);
        Typed::leaf(Ty::Bool, format!("(empty? {})", c.code))
    } else if roll < 94 {
        let b = gen_bool(ctx, depth - 1);
        let a = gen_bool(ctx, depth - 1);
        let c = gen_bool(ctx, depth - 1);
        Typed::leaf(Ty::Bool, format!("(if {} {} {})", b.code, a.code, c.code))
    } else {
        gen_call(ctx, depth, Ty::Bool).unwrap_or_else(|| Typed::leaf(Ty::Bool, "true".into()))
    }
}

fn gen_str(ctx: &mut Ctx, depth: u32) -> Typed {
    if depth == 0 || !ctx.spend() {
        let s = ctx.rng.pick(STR_POOL_WIDE);
        return Typed {
            ty: Ty::Str,
            code: escape_str(s),
            len: Some(s.chars().count()),
            lo: 0,
            hi: 0,
        };
    }
    if ctx.rng.chance(4) {
        return gen_do(ctx, depth, Ty::Str);
    }
    // AOT subset has no str-building calls: literals only there.
    if ctx.mode == GenMode::Aot {
        let s = ctx.rng.pick(STR_POOL_WIDE);
        return Typed {
            ty: Ty::Str,
            code: escape_str(s),
            len: Some(s.chars().count()),
            lo: 0,
            hi: 0,
        };
    }
    let roll = ctx.rng.below(100);
    if roll < 30 {
        let s = ctx.rng.pick(STR_POOL_WIDE);
        Typed {
            ty: Ty::Str,
            code: escape_str(s),
            len: Some(s.chars().count()),
            lo: 0,
            hi: 0,
        }
    } else if roll < 40 {
        let vs = ctx.vars_of(Ty::Str);
        if vs.is_empty() {
            let s = ctx.rng.pick(STR_POOL);
            Typed {
                ty: Ty::Str,
                code: escape_str(s),
                len: Some(s.len()),
                lo: 0,
                hi: 0,
            }
        } else {
            let v = ctx.rng.pick(&vs).clone();
            Typed {
                ty: Ty::Str,
                code: v.name,
                len: v.len,
                lo: 0,
                hi: 0,
            }
        }
    } else if roll < 62 {
        // (str a b ...) over printable scalars
        let n = ctx.rng.range(1, 4) as usize;
        let mut parts = Vec::new();
        for _ in 0..n {
            let e = match ctx.rng.below(4) {
                0 => gen_int(ctx, depth - 1),
                1 => gen_str(ctx, depth - 1),
                2 => gen_bool(ctx, depth - 1),
                _ => gen_vec(ctx, depth - 1),
            };
            parts.push(e.code);
        }
        Typed {
            ty: Ty::Str,
            code: format!("(str {})", parts.join(" ")),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 74 {
        // (subs s i j) on ASCII strings with literal in-range indices
        for _ in 0..8 {
            let s = gen_ascii_str(ctx, depth - 1);
            if let Some(n) = s.len {
                let i = ctx.rng.below(n as u64 + 1) as usize;
                let j = i + ctx.rng.below((n - i) as u64 + 1) as usize;
                return Typed {
                    ty: Ty::Str,
                    code: format!("(subs {} {i} {j})", s.code),
                    len: Some(j - i),
                    lo: 0,
                    hi: 0,
                };
            }
        }
        let s = ctx.rng.pick(STR_POOL);
        Typed {
            ty: Ty::Str,
            code: escape_str(s),
            len: Some(s.len()),
            lo: 0,
            hi: 0,
        }
    } else if roll < 80 {
        let e = gen_printable(ctx, depth - 1);
        Typed {
            ty: Ty::Str,
            code: format!("(pr-str {})", e.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 86 {
        let s = gen_str(ctx, depth - 1);
        let i = gen_int(ctx, depth - 1);
        Typed {
            ty: Ty::Str,
            code: format!("(format \"%s=%d\" {} {})", s.code, i.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else {
        gen_call(ctx, depth, Ty::Str).unwrap_or_else(|| Typed {
            ty: Ty::Str,
            code: escape_str("s"),
            len: Some(1),
            lo: 0,
            hi: 0,
        })
    }
}

/// ASCII-only strings (safe `subs` operands: indices are UTF-16 units).
fn gen_ascii_str(ctx: &mut Ctx, depth: u32) -> Typed {
    if depth == 0 || !ctx.spend() || ctx.rng.chance(70) {
        let s = ctx.rng.pick(STR_POOL);
        return Typed {
            ty: Ty::Str,
            code: escape_str(s),
            len: Some(s.len()),
            lo: 0,
            hi: 0,
        };
    }
    // (str a b) of ASCII parts stays ASCII with computable length.
    let a = gen_ascii_str(ctx, depth - 1);
    let b = gen_ascii_str(ctx, depth - 1);
    let len = match (a.len, b.len) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    };
    Typed {
        ty: Ty::Str,
        code: format!("(str {} {})", a.code, b.code),
        len,
        lo: 0,
        hi: 0,
    }
}

fn vec_literal(ctx: &mut Ctx, max_len: usize) -> Typed {
    let n = ctx.rng.below(max_len as u64 + 1) as usize;
    let mut els = Vec::new();
    for _ in 0..n {
        els.push(ctx.rng.range(-20, 100).to_string());
    }
    Typed {
        ty: Ty::Vec,
        code: format!("[{}]", els.join(" ")),
        len: Some(n),
        lo: 0,
        hi: 0,
    }
}

fn gen_vec(ctx: &mut Ctx, depth: u32) -> Typed {
    if depth == 0 || !ctx.spend() {
        return vec_literal(ctx, 5);
    }
    if ctx.rng.chance(4) {
        return gen_do(ctx, depth, Ty::Vec);
    }
    let roll = ctx.rng.below(100);
    if roll < 26 {
        vec_literal(ctx, 5)
    } else if roll < 34 {
        let vs = ctx.vars_of(Ty::Vec);
        if vs.is_empty() {
            vec_literal(ctx, 5)
        } else {
            let v = ctx.rng.pick(&vs).clone();
            Typed {
                ty: Ty::Vec,
                code: v.name,
                len: v.len,
                lo: 0,
                hi: 0,
            }
        }
    } else if roll < 48 {
        let v = gen_vec(ctx, depth - 1);
        let i = gen_int(ctx, depth - 1);
        let len = v.len.map(|n| n + 1);
        Typed {
            ty: Ty::Vec,
            code: format!("(conj {} {})", v.code, i.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 56 && ctx.mode == GenMode::Interp {
        // assoc/pop/subvec need known lengths (not AOT primitives)
        for _ in 0..8 {
            let v = gen_vec(ctx, depth - 1);
            if let Some(n) = v.len {
                let which = ctx.rng.below(3);
                if which == 0 && n > 0 {
                    let idx = ctx.rng.below(n as u64) as usize;
                    let i = gen_int(ctx, depth - 1);
                    return Typed {
                        ty: Ty::Vec,
                        code: format!("(assoc {} {idx} {})", v.code, i.code),
                        len: Some(n),
                        lo: 0,
                        hi: 0,
                    };
                } else if which == 1 && n > 0 {
                    return Typed {
                        ty: Ty::Vec,
                        code: format!("(pop {})", v.code),
                        len: Some(n - 1),
                        lo: 0,
                        hi: 0,
                    };
                } else if which == 2 {
                    let i = ctx.rng.below(n as u64 + 1) as usize;
                    let j = i + ctx.rng.below((n - i) as u64 + 1) as usize;
                    return Typed {
                        ty: Ty::Vec,
                        code: format!("(subvec {} {i} {j})", v.code),
                        len: Some(j - i),
                        lo: 0,
                        hi: 0,
                    };
                }
            }
        }
        vec_literal(ctx, 5)
    } else if roll < 64 && ctx.mode == GenMode::Interp {
        // (mapv f v): length preserved, ints stay ints
        let v = gen_vec(ctx, depth - 1);
        let f = *ctx.rng.pick(&["inc", "dec"]);
        Typed {
            ty: Ty::Vec,
            code: format!("(mapv {f} {})", v.code),
            len: v.len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 70 && ctx.mode == GenMode::Interp {
        // (filterv p v): unknown length
        let v = gen_vec(ctx, depth - 1);
        let p = *ctx.rng.pick(&["even?", "odd?"]);
        Typed {
            ty: Ty::Vec,
            code: format!("(filterv {p} {})", v.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 76 && ctx.mode == GenMode::Interp {
        // (vec s): length preserved from seq
        let s = gen_seq(ctx, depth - 1);
        Typed {
            ty: Ty::Vec,
            code: format!("(vec {})", s.code),
            len: s.len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 84 {
        let b = gen_bool(ctx, depth - 1);
        let a = gen_vec(ctx, depth - 1);
        let c = gen_vec(ctx, depth - 1);
        let len = match (a.len, c.len) {
            (Some(x), Some(y)) if x == y => Some(x),
            _ => None,
        };
        Typed {
            ty: Ty::Vec,
            code: format!("(if {} {} {})", b.code, a.code, c.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 90 {
        gen_let(ctx, depth, Ty::Vec)
    } else {
        gen_call(ctx, depth, Ty::Vec).unwrap_or_else(|| vec_literal(ctx, 5))
    }
}

fn gen_seq(ctx: &mut Ctx, depth: u32) -> Typed {
    debug_assert!(ctx.mode == GenMode::Interp, "gen_seq reached in AOT mode");
    if depth == 0 || !ctx.spend() {
        return range_seq(ctx);
    }
    let roll = ctx.rng.below(100);
    if roll < 24 {
        range_seq(ctx)
    } else if roll < 32 {
        let n = ctx.rng.range(0, 6);
        let x = gen_int(ctx, depth - 1);
        Typed {
            ty: Ty::Seq,
            code: format!("(repeat {n} {})", x.code),
            len: Some(n as usize),
            lo: 0,
            hi: 0,
        }
    } else if roll < 42 {
        let s = gen_seq(ctx, depth - 1);
        let n = ctx.rng.range(0, 6) as usize;
        let op = if ctx.rng.chance(50) { "take" } else { "drop" };
        let len = s.len.map(|l| {
            if op == "take" {
                l.min(n)
            } else {
                l.saturating_sub(n)
            }
        });
        Typed {
            ty: Ty::Seq,
            code: format!("({op} {n} {})", s.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 52 {
        let s = gen_seq(ctx, depth - 1);
        let lambda = ctx.rng.range(0, 4);
        Typed {
            ty: Ty::Seq,
            code: format!("(map inc {})", s.code),
            len: s.len,
            lo: 0,
            hi: 0,
        }
        .with_map_fn(ctx, lambda)
    } else if roll < 60 {
        let s = gen_seq(ctx, depth - 1);
        let p = *ctx.rng.pick(&["even?", "odd?"]);
        Typed {
            ty: Ty::Seq,
            code: format!("(filter {p} {})", s.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 68 {
        // (rest v): total; length - 1 when known
        let v = gen_vec(ctx, depth - 1);
        let len = v.len.map(|n| n.saturating_sub(1));
        Typed {
            ty: Ty::Seq,
            code: format!("(rest {})", v.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 76 {
        let i = gen_int(ctx, depth - 1);
        let s = gen_seq(ctx, depth - 1);
        let len = s.len.map(|n| n + 1);
        Typed {
            ty: Ty::Seq,
            code: format!("(cons {} {})", i.code, s.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 82 {
        let a = gen_seq(ctx, depth - 1);
        let b = gen_seq(ctx, depth - 1);
        let len = match (a.len, b.len) {
            (Some(x), Some(y)) => Some(x + y),
            _ => None,
        };
        Typed {
            ty: Ty::Seq,
            code: format!("(concat {} {})", a.code, b.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else if roll < 90 {
        let b = gen_bool(ctx, depth - 1);
        let a = gen_seq(ctx, depth - 1);
        let c = gen_seq(ctx, depth - 1);
        let len = match (a.len, c.len) {
            (Some(x), Some(y)) if x == y => Some(x),
            _ => None,
        };
        Typed {
            ty: Ty::Seq,
            code: format!("(if {} {} {})", b.code, a.code, c.code),
            len,
            lo: 0,
            hi: 0,
        }
    } else {
        gen_let(ctx, depth, Ty::Seq)
    }
}

impl Typed {
    fn with_map_fn(self, ctx: &mut Ctx, lambda: i64) -> Typed {
        let f = match lambda {
            0 => "inc".to_string(),
            1 => "dec".to_string(),
            _ => {
                let k = ctx.rng.range(-9, 10);
                format!("(fn [x] (+ x {k}))")
            }
        };
        Typed {
            code: self.code.replacen("map inc", &format!("map {f}"), 1),
            ..self
        }
    }
}

fn range_seq(ctx: &mut Ctx) -> Typed {
    let a = ctx.rng.range(0, 8);
    let b = a + ctx.rng.range(0, 7);
    Typed {
        ty: Ty::Seq,
        code: format!("(range {a} {b})"),
        len: Some((b - a) as usize),
        lo: 0,
        hi: 0,
    }
}

fn gen_map(ctx: &mut Ctx, depth: u32) -> Typed {
    debug_assert!(ctx.mode == GenMode::Interp, "gen_map reached in AOT mode");
    if depth == 0 || !ctx.spend() {
        return map_literal(ctx);
    }
    let roll = ctx.rng.below(100);
    if roll < 34 {
        map_literal(ctx)
    } else if roll < 44 {
        let vs = ctx.vars_of(Ty::Map);
        if vs.is_empty() {
            map_literal(ctx)
        } else {
            let v = ctx.rng.pick(&vs).clone();
            Typed {
                ty: Ty::Map,
                code: v.name,
                len: v.len,
                lo: 0,
                hi: 0,
            }
        }
    } else if roll < 62 {
        let m = gen_map(ctx, depth - 1);
        let k = format!(":{}", ctx.rng.pick(KW_POOL));
        // Small literal values keep (get m k default) ranges tight.
        let v = ctx.rng.range(-99, 100);
        Typed {
            ty: Ty::Map,
            code: format!("(assoc {} {k} {v})", m.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 74 {
        let m = gen_map(ctx, depth - 1);
        let k = format!(":{}", ctx.rng.pick(KW_POOL));
        Typed {
            ty: Ty::Map,
            code: format!("(dissoc {} {k})", m.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 84 {
        let a = gen_map(ctx, depth - 1);
        let b = gen_map(ctx, depth - 1);
        Typed {
            ty: Ty::Map,
            code: format!("(merge {} {})", a.code, b.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else {
        gen_let(ctx, depth, Ty::Map)
    }
}

fn map_literal(ctx: &mut Ctx) -> Typed {
    let n = ctx.rng.below(5) as usize;
    let mut keys: Vec<&str> = KW_POOL.to_vec();
    // Fisher-Yates with our own rng for determinism.
    for i in (1..keys.len()).rev() {
        let j = ctx.rng.below(i as u64 + 1) as usize;
        keys.swap(i, j);
    }
    let mut pairs = Vec::new();
    for k in keys.iter().take(n) {
        pairs.push(format!(":{k} {}", ctx.rng.range(-20, 100)));
    }
    Typed {
        ty: Ty::Map,
        code: format!("{{{}}}", pairs.join(" ")),
        len: Some(n),
        lo: 0,
        hi: 0,
    }
}

fn gen_set(ctx: &mut Ctx, depth: u32) -> Typed {
    debug_assert!(ctx.mode == GenMode::Interp, "gen_set reached in AOT mode");
    if depth == 0 || !ctx.spend() {
        return set_literal(ctx);
    }
    let roll = ctx.rng.below(100);
    if roll < 36 {
        set_literal(ctx)
    } else if roll < 46 {
        let vs = ctx.vars_of(Ty::Set);
        if vs.is_empty() {
            set_literal(ctx)
        } else {
            let v = ctx.rng.pick(&vs).clone();
            Typed {
                ty: Ty::Set,
                code: v.name,
                len: v.len,
                lo: 0,
                hi: 0,
            }
        }
    } else if roll < 64 {
        let s = gen_set(ctx, depth - 1);
        let i = gen_int(ctx, depth - 1);
        Typed {
            ty: Ty::Set,
            code: format!("(conj {} {})", s.code, i.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 76 {
        let s = gen_set(ctx, depth - 1);
        let i = gen_int(ctx, depth - 1);
        Typed {
            ty: Ty::Set,
            code: format!("(disj {} {})", s.code, i.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else if roll < 86 {
        let v = gen_vec(ctx, depth - 1);
        Typed {
            ty: Ty::Set,
            code: format!("(set {})", v.code),
            len: None,
            lo: 0,
            hi: 0,
        }
    } else {
        gen_let(ctx, depth, Ty::Set)
    }
}

fn set_literal(ctx: &mut Ctx) -> Typed {
    // Unique values: draw from a shuffled pool.
    let mut pool: Vec<i64> = (-5..25).collect();
    for i in (1..pool.len()).rev() {
        let j = ctx.rng.below(i as u64 + 1) as usize;
        pool.swap(i, j);
    }
    let n = ctx.rng.below(5) as usize;
    let els: Vec<String> = pool.iter().take(n).map(|v| v.to_string()).collect();
    Typed {
        ty: Ty::Set,
        code: format!("#{{{}}}", els.join(" ")),
        len: Some(n),
        lo: 0,
        hi: 0,
    }
}

/// Any collection/string with a (possibly unknown) count
/// (AOT: vectors and strings only).
fn gen_sized(ctx: &mut Ctx, depth: u32) -> Typed {
    if ctx.mode == GenMode::Aot {
        return if ctx.rng.chance(60) {
            gen_vec(ctx, depth)
        } else {
            gen_str(ctx, depth)
        };
    }
    match ctx.rng.below(6) {
        0 => gen_vec(ctx, depth),
        1 => gen_seq(ctx, depth),
        2 => gen_str(ctx, depth),
        3 => gen_map(ctx, depth),
        4 => gen_set(ctx, depth),
        _ => gen_vec(ctx, depth),
    }
}

/// Printable scalar/collection for `str` args, `pr-str`, and `do` sides
/// (order-defined; AOT: no seqs).
fn gen_printable(ctx: &mut Ctx, depth: u32) -> Typed {
    match ctx.rng.below(5) {
        0 => gen_int(ctx, depth),
        1 => gen_bool(ctx, depth),
        2 => gen_str(ctx, depth),
        3 => gen_vec(ctx, depth),
        _ if ctx.mode == GenMode::Interp => gen_seq(ctx, depth),
        _ => gen_int(ctx, depth),
    }
}

/// (do side value): sequence-point torture (in-subset for both modes).
fn gen_do(ctx: &mut Ctx, depth: u32, ret: Ty) -> Typed {
    let side = gen_printable(ctx, depth.saturating_sub(1));
    let val = gen_by_ty(ctx, ret, depth.saturating_sub(1));
    Typed {
        ty: ret,
        code: format!("(do {} {})", side.code, val.code),
        len: val.len,
        lo: val.lo,
        hi: val.hi,
    }
}

fn gen_by_ty(ctx: &mut Ctx, ty: Ty, depth: u32) -> Typed {
    match ty {
        Ty::Int => gen_int(ctx, depth),
        Ty::Bool => gen_bool(ctx, depth),
        Ty::Str => gen_str(ctx, depth),
        Ty::Vec => gen_vec(ctx, depth),
        Ty::Seq => gen_seq(ctx, depth),
        Ty::Map => gen_map(ctx, depth),
        Ty::Set => gen_set(ctx, depth),
    }
}

fn gen_let(ctx: &mut Ctx, depth: u32, ret: Ty) -> Typed {
    let n = ctx.rng.range(1, 4) as usize;
    let mut bindings = Vec::new();
    let mut new_vars = Vec::new();
    for _ in 0..n {
        // Binding types: maps/sets only in interp mode.
        let ty = match ctx
            .rng
            .below(if ctx.mode == GenMode::Interp { 7 } else { 4 })
        {
            0 => Ty::Int,
            1 => Ty::Bool,
            2 => Ty::Str,
            3 => Ty::Vec,
            4 => Ty::Seq,
            5 => Ty::Map,
            _ => Ty::Set,
        };
        let e = gen_by_ty(ctx, ty, depth.saturating_sub(1));
        let name = ctx.fresh_var();
        bindings.push(format!("{name} {}", e.code));
        new_vars.push(Var {
            name,
            ty,
            len: e.len,
            lo: e.lo,
            hi: e.hi,
        });
    }
    // Bindings are generated against the outer scope only (parallel-let
    // style: always valid, slightly less tangled than sequential lets).
    let saved = std::mem::take(&mut ctx.vars);
    let mut all = saved.clone();
    all.extend(new_vars.iter().cloned());
    ctx.vars = all;
    let body = gen_by_ty(ctx, ret, depth.saturating_sub(1));
    ctx.vars = saved;
    Typed {
        ty: ret,
        code: format!("(let [{}] {})", bindings.join(" "), body.code),
        len: body.len,
        lo: body.lo,
        hi: body.hi,
    }
}

/// Terminating loop: literal bound, int accumulator.
fn gen_loop(ctx: &mut Ctx, depth: u32) -> Typed {
    debug_assert!(ctx.mode == GenMode::Interp, "gen_loop reached in AOT mode");
    let bound = ctx.rng.range(1, 13);
    let acc0 = ctx.rng.range(-20, 50);
    let iname = ctx.fresh_var();
    let aname = ctx.fresh_var();
    let saved = std::mem::take(&mut ctx.vars);
    ctx.vars = saved.clone();
    ctx.vars.push(Var {
        name: iname.clone(),
        ty: Ty::Int,
        len: None,
        lo: 0,
        hi: bound,
    });
    ctx.vars.push(Var {
        name: aname.clone(),
        ty: Ty::Int,
        len: None,
        lo: -RANGE_CAP,
        hi: RANGE_CAP,
    });
    let update = gen_int(ctx, depth.saturating_sub(1));
    ctx.vars = saved;
    let (lo, hi) = (acc0.min(update.lo), acc0.max(update.hi));
    let (lo, hi) = (
        lo.clamp(-RANGE_CAP, RANGE_CAP),
        hi.clamp(-RANGE_CAP, RANGE_CAP),
    );
    Typed::int(
        format!(
            "(loop [{iname} 0 {aname} {acc0}] (if (< {iname} {bound}) (recur (inc {iname}) {u}) {aname}))",
            u = update.code
        ),
        lo,
        hi,
    )
}

/// Call an already-defined fn returning `ret`, or None if there is none.
/// Recursive fns are excluded: arbitrary call-site args could diverge.
fn gen_call(ctx: &mut Ctx, depth: u32, ret: Ty) -> Option<Typed> {
    let cands: Vec<FnDef> = ctx
        .fns
        .iter()
        .filter(|f| f.ret == ret && !f.recursive)
        .cloned()
        .collect();
    if cands.is_empty() {
        return None;
    }
    let f = ctx.rng.pick(&cands).clone();
    let mut args = Vec::new();
    for p in &f.params {
        args.push(gen_by_ty(ctx, p.ty, depth.saturating_sub(1)).code);
    }
    // Calls return unknown ranges/lengths (conservative).
    Some(Typed {
        ty: ret,
        code: format!("({} {})", f.name, args.join(" ")).replace(" )", ")"),
        len: None,
        lo: -RANGE_CAP,
        hi: RANGE_CAP,
    })
}

// ------------------------------------------------------- top level ---

fn fn_name(ctx: &mut Ctx) -> String {
    let n = ctx.fn_counter;
    ctx.fn_counter += 1;
    format!("f{n}")
}

fn gen_fn_def(ctx: &mut Ctx, allow_vec_str: bool) -> FnDef {
    let n_params = ctx.rng.range(0, 4) as usize;
    let mut params = Vec::new();
    for i in 0..n_params {
        let ty = match ctx.rng.below(if allow_vec_str { 4 } else { 1 }) {
            0 => Ty::Int,
            1 => Ty::Bool,
            2 => Ty::Str,
            _ => Ty::Vec,
        };
        params.push(FnParam {
            name: format!("p{i}"),
            ty,
        });
    }
    let ret = match ctx.rng.below(if allow_vec_str { 4 } else { 1 }) {
        0 => Ty::Int,
        1 => Ty::Bool,
        2 => Ty::Str,
        _ => Ty::Vec,
    };
    let saved = std::mem::take(&mut ctx.vars);
    ctx.vars = params
        .iter()
        .map(|p| Var {
            name: p.name.clone(),
            ty: p.ty,
            len: None,
            lo: -RANGE_CAP,
            hi: RANGE_CAP,
        })
        .collect();
    let body = gen_by_ty(ctx, ret, ctx.max_depth);
    ctx.vars = saved;
    FnDef {
        name: fn_name(ctx),
        params,
        ret,
        body: body.code,
        recursive: false,
    }
}

/// A self-recursive int fn (fact/fib/sumdown shape) + a safe call argument.
fn gen_recursive_fn(ctx: &mut Ctx) -> (FnDef, String) {
    let name = fn_name(ctx);
    let shape = ctx.rng.below(3);
    let (body, arg) = match shape {
        0 => {
            let arg = ctx.rng.range(0, 9);
            (
                format!("(if (= n 0) 1 (* n ({name} (+ n -1))))"),
                arg.to_string(),
            )
        }
        1 => {
            let arg = ctx.rng.range(0, 13);
            (
                format!("(if (< n 2) n (+ ({name} (+ n -1)) ({name} (+ n -2))))"),
                arg.to_string(),
            )
        }
        _ => {
            let arg = ctx.rng.range(0, 21);
            (
                format!("(if (= n 0) 0 (+ n ({name} (+ n -1))))"),
                arg.to_string(),
            )
        }
    };
    let def = FnDef {
        name: name.clone(),
        params: vec![FnParam {
            name: "n".into(),
            ty: Ty::Int,
        }],
        ret: Ty::Int,
        body,
        recursive: true,
    };
    (def, format!("({name} {arg})"))
}

fn render_fn(f: &FnDef) -> String {
    let params: Vec<&str> = f.params.iter().map(|p| p.name.as_str()).collect();
    format!("(defn {} [{}] {})", f.name, params.join(" "), f.body)
}

fn gen_result_vec(ctx: &mut Ctx) -> String {
    let n = ctx.rng.range(2, 6) as usize;
    let mut items = Vec::new();
    for _ in 0..n {
        let e = match ctx.rng.below(5) {
            0 => gen_int(ctx, ctx.max_depth),
            1 => gen_bool(ctx, ctx.max_depth),
            2 => gen_str(ctx, ctx.max_depth),
            3 => gen_vec(ctx, ctx.max_depth),
            _ if ctx.mode == GenMode::Interp => gen_seq(ctx, ctx.max_depth),
            _ => gen_int(ctx, ctx.max_depth),
        };
        items.push(e.code);
    }
    format!("[{}]", items.join(" "))
}

/// Generate one complete program.
pub fn generate_program(mode: GenMode, seed: u64, max_depth: u32) -> String {
    let mut ctx = Ctx {
        rng: Rng(seed.wrapping_add(0x9E3779B97F4A7C15)),
        mode,
        max_depth,
        vars: Vec::new(),
        fns: Vec::new(),
        var_counter: 0,
        fn_counter: 0,
        fuel: 400,
    };
    let mut out = String::new();
    let mode_name = match mode {
        GenMode::Interp => "interp",
        GenMode::Aot => "aot",
    };
    out.push_str(&format!(
        ";; generated by clj-torture generate --mode {mode_name} --seed {seed}\n"
    ));
    out.push_str(";; { dg-do run }\n");
    match mode {
        GenMode::Interp => out.push_str(";; { dg-mode interp }\n"),
        GenMode::Aot => out.push_str(";; { dg-mode interp+aot }\n"),
    }
    out.push('\n');
    match mode {
        GenMode::Interp => {
            let n_fns = ctx.rng.below(3) as usize;
            for _ in 0..n_fns {
                let f = gen_fn_def(&mut ctx, true);
                out.push_str(&render_fn(&f));
                out.push('\n');
                ctx.fns.push(f);
            }
            let result = gen_result_vec(&mut ctx);
            out.push_str(&format!("(prn {result})\n"));
        }
        GenMode::Aot => {
            // 1-3 fns; at most one recursive (no mutual recursion in-subset).
            let n_fns = ctx.rng.range(1, 4) as usize;
            let rec_at = ctx.rng.below(n_fns as u64) as usize;
            let mut rec_call = String::new();
            for i in 0..n_fns {
                if i == rec_at && ctx.rng.chance(60) {
                    let (f, call) = gen_recursive_fn(&mut ctx);
                    out.push_str(&render_fn(&f));
                    out.push('\n');
                    rec_call = call;
                    ctx.fns.push(f);
                } else {
                    let f = gen_fn_def(&mut ctx, true);
                    out.push_str(&render_fn(&f));
                    out.push('\n');
                    ctx.fns.push(f);
                }
            }
            let mut result = gen_result_vec(&mut ctx);
            if !rec_call.is_empty() {
                // Splice the recursive call into the result vector.
                result.pop(); // trailing ]
                result.push_str(&format!(" {rec_call}]"));
            }
            out.push_str(&format!("(print {result})\n"));
        }
    }
    out
}

pub fn generate(args: GenArgs) -> i32 {
    let mode = match GenMode::parse(&args.mode) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    if args.out.is_none() && args.count != 1 {
        eprintln!("--count N > 1 requires --out DIR");
        return 2;
    }
    for i in 0..args.count {
        let seed = args.seed + i;
        let program = generate_program(mode, seed, args.depth);
        if let Some(dir) = &args.out {
            if let Err(e) = std::fs::create_dir_all(dir) {
                eprintln!("cannot create {}: {e}", dir.display());
                return 2;
            }
            let path = dir.join(format!("seed-{seed}.clj"));
            if let Err(e) = std::fs::write(&path, &program) {
                eprintln!("cannot write {}: {e}", path.display());
                return 2;
            }
        } else {
            print!("{program}");
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        for mode in [GenMode::Interp, GenMode::Aot] {
            for seed in [1, 2, 42, 999] {
                let a = generate_program(mode, seed, 4);
                let b = generate_program(mode, seed, 4);
                assert_eq!(a, b, "mode {mode:?} seed {seed}");
            }
        }
    }

    #[test]
    fn seeds_differ() {
        let a = generate_program(GenMode::Interp, 1, 4);
        let b = generate_program(GenMode::Interp, 2, 4);
        assert_ne!(a, b);
    }

    /// Cheap paren/brace/bracket balance check (string/char/comment aware).
    fn balanced(program: &str) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut chars = program.chars().peekable();
        let mut in_string = false;
        while let Some(c) = chars.next() {
            if in_string {
                if c == '\\' {
                    chars.next();
                } else if c == '"' {
                    in_string = false;
                }
                continue;
            }
            match c {
                ';' => {
                    for nc in chars.by_ref() {
                        if nc == '\n' {
                            break;
                        }
                    }
                }
                '"' => in_string = true,
                '\\' => {
                    chars.next(); // char literal: skip next char
                }
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
                _ => {}
            }
        }
        !in_string && stack.is_empty()
    }

    #[test]
    fn programs_balanced() {
        for mode in [GenMode::Interp, GenMode::Aot] {
            for seed in 1..=60 {
                let p = generate_program(mode, seed, 4);
                assert!(balanced(&p), "unbalanced: mode {mode:?} seed {seed}:\n{p}");
            }
        }
    }

    #[test]
    fn aot_subset_shape() {
        // AOT programs must not contain out-of-subset operations.
        let banned = [
            "loop",
            "recur",
            "defmacro",
            "apply",
            "map",
            "filter",
            "mapv",
            "filterv",
            "str",
            "subs",
            "format",
            "pr-str",
            "println",
            "prn",
            "first",
            "second",
            "last",
            "rest",
            "next",
            "range",
            "repeat",
            "take",
            "drop",
            "merge",
            "dissoc",
            "assoc",
            "pop",
            "subvec",
            "contains?",
            "empty?",
            "inc",
            "dec",
            "abs",
            "min",
            "max",
            "rem",
            "mod",
            "cons",
            "concat",
            "comp",
            "partial",
            "letfn",
            "doall",
            "if-let",
            "and",
            "or",
            "not",
            "zero?",
            "pos?",
            "neg?",
            "even?",
            "odd?",
            "vec",
            "set",
            "seq",
            "conj!",
            "fn",
            "when",
            "cond",
            "case",
            "declare",
            "assert",
            "true?",
        ];
        for seed in 1..=60 {
            let p = generate_program(GenMode::Aot, seed, 4);
            for word in banned {
                // Word-ish check: banned token followed by space/paren/end.
                let mut idx = 0;
                while let Some(pos) = p[idx..].find(word) {
                    let abs = idx + pos;
                    let after = p[abs + word.len()..].chars().next();
                    let before = p[..abs].chars().last();
                    let wordy = |c: Option<char>| {
                        c.map(|c| c.is_alphanumeric() || c == '-' || c == '?' || c == '!')
                            .unwrap_or(false)
                    };
                    if !wordy(before) && !wordy(after) {
                        panic!("AOT seed {seed} uses banned {word}:\n{p}");
                    }
                    idx = abs + word.len();
                }
            }
            assert!(
                p.contains("(print ["),
                "AOT seed {seed} has no print top:\n{p}"
            );
        }
    }

    #[test]
    fn interp_prints_result() {
        for seed in 1..=20 {
            let p = generate_program(GenMode::Interp, seed, 4);
            assert!(p.contains("(prn ["), "seed {seed}:\n{p}");
        }
    }
}
