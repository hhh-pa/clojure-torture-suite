//! DejaGNU-style `{ dg-* }` directive parsing.
//!
//! Directives live in `;;` comments on leading comment lines (blank lines
//! and `;` comments before the first code form). Unknown `dg-` directives
//! are errors, like DejaGNU.

use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoWhat {
    Run,
    Load,
    Read,
    FailRun,
    FailLoad,
    FailRead,
}

impl DoWhat {
    /// True when the harness demands exit 0 (vs. nonzero for `fail-*`).
    pub fn expect_zero(self) -> bool {
        matches!(self, DoWhat::Run | DoWhat::Load | DoWhat::Read)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Interp,
    Aot,
    InterpAot,
}

#[derive(Debug, Clone)]
pub struct Directives {
    pub do_what: DoWhat,
    pub mode: Mode,
    /// Exact expected stdout bytes (`dg-output`, C escapes interpreted).
    pub output: Option<Vec<u8>>,
    /// Stdout regex (`dg-output-re`, Rust regex syntax, unanchored search).
    pub output_re: Option<String>,
    /// Stderr regex for `fail-*` tests (`dg-error-re`).
    pub error_re: Option<String>,
    /// Per-test timeout in seconds (`dg-timeout`).
    pub timeout: Option<u64>,
    /// Backends expected to fail (`dg-xfail-backend`).
    pub xfail: Vec<String>,
    /// Backends to skip (`dg-skip-backend`).
    pub skip: Vec<String>,
    /// Script argv (`dg-args`, shell-split).
    pub args: Vec<String>,
}

impl Default for Directives {
    fn default() -> Self {
        Directives {
            do_what: DoWhat::Run,
            mode: Mode::Interp,
            output: None,
            output_re: None,
            error_re: None,
            timeout: None,
            xfail: Vec::new(),
            skip: Vec::new(),
            args: Vec::new(),
        }
    }
}

fn dg_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"\{\s*(dg-[a-z-]+)\s*(.*?)\s*\}").unwrap())
}

/// Interpret C escapes (`\n \t \r \\ \" \0`; unknown escapes keep the char).
pub fn c_unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            None => out.push('\\'),
            Some(n) => out.push(match n {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '"' => '"',
                '0' => '\0',
                other => other,
            }),
        }
    }
    out
}

fn unquote(value: &str, path: &str) -> Result<String, String> {
    let value = value.trim();
    if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
        Ok(value[1..value.len() - 1].to_string())
    } else {
        Err(format!(
            "{path}: expected double-quoted string, got {value:?}"
        ))
    }
}

/// Minimal shell split: whitespace separation, `"..."` grouping, `\` escapes.
pub fn shell_split(s: &str) -> Result<Vec<String>, String> {
    let mut words = Vec::new();
    let mut cur = String::new();
    let mut in_word = false;
    let mut in_quote = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quote {
            if c == '"' {
                in_quote = false;
            } else if c == '\\' {
                match chars.next() {
                    Some(n) => cur.push(n),
                    None => return Err("unterminated escape in dg-args".into()),
                }
            } else {
                cur.push(c);
            }
        } else if c == '"' {
            in_quote = true;
            in_word = true;
        } else if c == '\\' {
            match chars.next() {
                Some(n) => {
                    cur.push(n);
                    in_word = true;
                }
                None => return Err("unterminated escape in dg-args".into()),
            }
        } else if c.is_whitespace() {
            if in_word {
                words.push(std::mem::take(&mut cur));
                in_word = false;
            }
        } else {
            cur.push(c);
            in_word = true;
        }
    }
    if in_quote {
        return Err("unterminated quote in dg-args".into());
    }
    if in_word {
        words.push(cur);
    }
    Ok(words)
}

fn check_backend(name: &str, path: &str, directive: &str) -> Result<(), String> {
    if name == "jvm" || name == "clojure-rs" {
        Ok(())
    } else {
        Err(format!("{path}: bad backend '{name}' in {directive}"))
    }
}

pub fn parse_str(path: &str, text: &str) -> Result<Directives, String> {
    let mut d = Directives::default();
    for line in text.lines() {
        let stripped = line.trim();
        if !(stripped.is_empty() || stripped.starts_with(';')) {
            break; // first code form: directives end here
        }
        for cap in dg_pattern().captures_iter(line) {
            let name = &cap[1];
            let value = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            match name {
                "dg-do" => {
                    d.do_what = match value.trim() {
                        "run" => DoWhat::Run,
                        "load" => DoWhat::Load,
                        "read" => DoWhat::Read,
                        "fail-run" => DoWhat::FailRun,
                        "fail-load" => DoWhat::FailLoad,
                        "fail-read" => DoWhat::FailRead,
                        other => {
                            return Err(format!("{path}: bad dg-do '{other}'"));
                        }
                    }
                }
                "dg-mode" => {
                    d.mode = match value.trim() {
                        "interp" => Mode::Interp,
                        "aot" => Mode::Aot,
                        "interp+aot" => Mode::InterpAot,
                        other => {
                            return Err(format!("{path}: bad dg-mode '{other}'"));
                        }
                    }
                }
                "dg-output" => d.output = Some(c_unescape(&unquote(value, path)?).into_bytes()),
                "dg-output-re" => d.output_re = Some(c_unescape(&unquote(value, path)?)),
                "dg-error-re" => d.error_re = Some(c_unescape(&unquote(value, path)?)),
                "dg-timeout" => {
                    d.timeout = Some(
                        value
                            .trim()
                            .parse::<u64>()
                            .map_err(|_| format!("{path}: bad dg-timeout '{value}'"))?,
                    )
                }
                "dg-xfail-backend" => {
                    for b in value.split_whitespace() {
                        check_backend(b, path, "dg-xfail-backend")?;
                        d.xfail.push(b.to_string());
                    }
                }
                "dg-skip-backend" => {
                    for b in value.split_whitespace() {
                        check_backend(b, path, "dg-skip-backend")?;
                        d.skip.push(b.to_string());
                    }
                }
                "dg-args" => d.args = shell_split(value).map_err(|e| format!("{path}: {e}"))?,
                other => return Err(format!("{path}: unknown directive {other}")),
            }
        }
    }
    if d.output.is_some() && d.output_re.is_some() {
        return Err(format!("{path}: dg-output and dg-output-re are exclusive"));
    }
    if d.timeout == Some(0) {
        d.timeout = None;
    }
    Ok(d)
}

pub fn parse_file(path: &std::path::Path) -> Result<Directives, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    parse_str(&path.display().to_string(), &text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let d = parse_str("t.clj", "(println 1)\n").unwrap();
        assert_eq!(d.do_what, DoWhat::Run);
        assert_eq!(d.mode, Mode::Interp);
        assert!(d.output.is_none());
    }

    #[test]
    fn full_directives() {
        let d = parse_str(
            "t.clj",
            ";; comment\n;; { dg-do run } { dg-mode interp+aot }\n;; { dg-output \"A\\n\" }\n;; { dg-timeout 7 }\n;; { dg-args a \"b c\" }\n(+ 1 2)\n",
        )
        .unwrap();
        assert_eq!(d.do_what, DoWhat::Run);
        assert_eq!(d.mode, Mode::InterpAot);
        assert_eq!(d.output, Some(b"A\n".to_vec()));
        assert_eq!(d.timeout, Some(7));
        assert_eq!(d.args, vec!["a".to_string(), "b c".to_string()]);
    }

    #[test]
    fn stops_at_code() {
        // A dg-looking comment after code is not a directive.
        let d = parse_str("t.clj", "(+ 1 2)\n;; { dg-do fail-run }\n").unwrap();
        assert_eq!(d.do_what, DoWhat::Run);
    }

    #[test]
    fn unknown_directive_errors() {
        assert!(parse_str("t.clj", ";; { dg-frob 1 }\n").is_err());
    }

    #[test]
    fn output_forms_exclusive() {
        assert!(parse_str(
            "t.clj",
            ";; { dg-output \"a\" }\n;; { dg-output-re \"a\" }\n"
        )
        .is_err());
    }

    #[test]
    fn bad_backend_rejected() {
        assert!(parse_str("t.clj", ";; { dg-skip-backend jvmx }\n").is_err());
        let d = parse_str("t.clj", ";; { dg-xfail-backend clojure-rs }\n").unwrap();
        assert_eq!(d.xfail, vec!["clojure-rs".to_string()]);
    }

    #[test]
    fn shell_split_cases() {
        assert_eq!(shell_split("a \"b c\" d").unwrap(), vec!["a", "b c", "d"]);
        assert!(shell_split("a \"b").is_err());
    }

    #[test]
    fn unescape_cases() {
        assert_eq!(c_unescape("A\\nB\\t\\\\\\\""), "A\nB\t\\\"");
    }
}
