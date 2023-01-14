use rail_lang::{
    rail_machine::{self, RailState},
    RunConventions,
};
use rustyline::{error::ReadlineError, Editor};

use crate::{run, LogLevel, Module};

pub struct StapPrompt {
    editor: Editor<()>,
    conventions: RunConventions<'static>,
    log_level: LogLevel,
}

impl StapPrompt {
    pub fn new(conventions: RunConventions<'static>, log_level: LogLevel) -> Self {
        let editor = Editor::<()>::new().expect("Unable to boot editor");
        StapPrompt {
            editor,
            conventions,
            log_level,
        }
    }

    pub fn run(&mut self, state: RailState) -> RailState {
        let mut state = state;

        let mut input = String::new();

        loop {
            input = match self.editor.readline("> ") {
                Err(e) => {
                    // ^D and ^C are not error cases.
                    if let ReadlineError::Eof = e {
                        rail_machine::log_fatal(&self.conventions, "End of input");
                        return state;
                    } else if let ReadlineError::Interrupted = e {
                        rail_machine::log_fatal(&self.conventions, "Process interrupt");
                        return state;
                    }

                    eprintln!("Something bad happened. {:?}", e);
                    std::process::exit(1);
                }
                Ok(line) => input + &line,
            };

            let (module, remaining) = Module::parse_line(&input);
            input = remaining;

            state = run(state, module, self.log_level);
        }
    }
}
