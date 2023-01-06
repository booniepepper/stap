use rail_lang::{
    rail_machine::{self, RailState},
    RunConventions,
};
use rustyline::{error::ReadlineError, Editor};

use crate::{run, Module};

pub struct StapPrompt {
    editor: Editor<()>,
    conventions: RunConventions<'static>,
}

impl StapPrompt {
    pub fn new(conventions: RunConventions<'static>) -> Self {
        let editor = Editor::<()>::new().expect("Unable to boot editor");
        StapPrompt {
            editor,
            conventions,
        }
    }

    pub fn run(&mut self, state: RailState) {
        let mut state = state;
        loop {
            let input = self.editor.readline("> ");

            if let Err(e) = input {
                // ^D and ^C are not error cases.
                if let ReadlineError::Eof = e {
                    rail_machine::log_fatal(&self.conventions, "End of input");
                    return;
                } else if let ReadlineError::Interrupted = e {
                    rail_machine::log_fatal(&self.conventions, "Process interrupt");
                    return;
                }

                eprintln!("Something bad happened. {:?}", e);
                std::process::exit(1);
            }

            let input = input.unwrap();

            let module = Module::parse(&input);

            state = run(state, module);
        }
    }
}
