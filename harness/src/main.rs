//! clj-torture: Clojure torture suite runner, program generator, fuzzer.

mod directives;
mod fuzz;
mod gen;
mod runner;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "clj-torture",
    version,
    about = "Clojure torture suite: run the corpus, generate programs, fuzz compilers"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the torture corpus (torture/**/*.clj) on selected backends
    Run(runner::RunArgs),
    /// Generate random torture programs (Csmith-style, seeded)
    Generate(gen::GenArgs),
    /// Differentially fuzz backends with generated programs
    Fuzz(fuzz::FuzzArgs),
}

fn main() {
    let cli = Cli::parse();
    let code = match cli.command {
        Command::Run(args) => runner::run(args),
        Command::Generate(args) => gen::generate(args),
        Command::Fuzz(args) => fuzz::fuzz(args),
    };
    std::process::exit(code);
}
