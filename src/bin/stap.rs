use rail_lang::{corelib::rail_builtin_dictionary, rail_machine::RailState, RunConventions};
use stap::{run, Module};

pub const STAP_VERSION: &str = std::env!("CARGO_PKG_VERSION");
pub const STAP_CONVENTIONS: RunConventions = RunConventions {
    exe_name: "stap",
    exe_version: STAP_VERSION,
    warn_prefix: "WARN",
    fatal_prefix: "STOP",
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: stap FILE");
        std::process::exit(1);
    }

    let _exe_name = &args[0];
    let source_file = &args[1];

    let content = match std::fs::read_to_string(source_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Unable to read {}. {:?}", source_file, e);
            std::process::exit(1);
        }
    };

    let module = Module::parse(&content);

    let exit_code = run(
        RailState::new_main(rail_builtin_dictionary(), &STAP_CONVENTIONS),
        module,
    );

    std::process::exit(exit_code);
}
