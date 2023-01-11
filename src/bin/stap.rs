use clap::{Parser, Subcommand};
use rail_lang::{
    corelib::rail_builtin_dictionary,
    rail_machine::{self, RailState},
    RunConventions,
};
use stap::{LogLevel, Module, StapPrompt};

pub const STAP_VERSION: &str = std::env!("CARGO_PKG_VERSION");
pub const STAP_CONVENTIONS: RunConventions = RunConventions {
    exe_name: "stap",
    exe_version: STAP_VERSION,
    warn_prefix: "WARN",
    fatal_prefix: "STOP",
};

fn main() {
    let args = StapArgs::parse();

    let state = RailState::new_main(rail_builtin_dictionary(), &STAP_CONVENTIONS);
    let log_level = args.log.unwrap_or(LogLevel::WhateverRailDoes);

    let end_state = match args.mode {
        Some(Mode::Interactive) | None => StapPrompt::new(STAP_CONVENTIONS, log_level).run(state),
        Some(Mode::Run { file }) => {
            let content = match std::fs::read_to_string(&file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Unable to read {}. {:?}", file, e);
                    std::process::exit(1);
                }
            };
            let module = Module::parse(&content);
            stap::run(state, module, log_level)
        }
    };

    if !end_state.stack.is_empty() {
        rail_machine::log_warn(&STAP_CONVENTIONS, end_state.stack);
    }

    std::process::exit(0)
}

#[derive(Parser)]
#[command(name = "stap", version = STAP_VERSION)]
/// stap is an experimental concatenative Lisp based on Rail
struct StapArgs {
    #[command(subcommand)]
    mode: Option<Mode>,

    #[arg(long)]
    log: Option<LogLevel>,
}

#[derive(Subcommand)]
enum Mode {
    #[command(visible_alias = "i")]
    /// Start and interactive session. (Default when no subcommand specified)
    Interactive,

    #[command(visible_alias = "f")]
    /// Run a file.
    Run { file: String },
}
