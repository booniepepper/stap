mod parser;

use clap::ValueEnum;
pub use parser::*;
mod prompt;
pub use prompt::*;

use rail_lang::{rail_machine::RailState, tokens::Token};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum LogLevel {
    WhateverRailDoes,
    Trace,
}

pub fn run(state: RailState, module: Module, log_level: LogLevel) -> RailState {
    let rail_tokens = module
        .values
        .into_iter()
        .flat_map(to_rail_tokens(log_level))
        .collect::<Vec<_>>();

    state.run_tokens(rail_tokens)
}

fn to_rail_tokens(log_level: LogLevel) -> impl Fn(Value) -> Vec<Token> {
    move |value: Value| {
        let tokens = match value.clone() {
            Value::String(s) => vec![Token::String(s)],
            Value::Identifier(i) => {
                let token = match Token::from(i) {
                    Token::Term(t) => Token::DeferredTerm(t),
                    t => t,
                };
                vec![token]
            }
            Value::List(vals) => {
                let mut tokens = vec![Token::LeftBracket];
                vals.into_iter()
                    .flat_map(to_rail_tokens(log_level))
                    .for_each(|t| tokens.push(t));
                tokens.push(Token::RightBracket);
                tokens
            }
            Value::Function(args) => {
                let mut tokens = match args.len() {
                    0 => vec![],
                    1 => to_rail_tokens(log_level)(args[0].clone()),
                    _ => args
                        .into_iter()
                        .rev()
                        .flat_map(to_rail_tokens(log_level))
                        .collect(),
                };

                if let Some(token) = tokens.last_mut() {
                    if let Token::DeferredTerm(term) = token {
                        *token = Token::Term(term.to_owned());
                    }
                }

                tokens
            }
        };

        if let LogLevel::Trace = log_level {
            eprintln!("DEBUG to_rail_tokens({:?}) = {:?}", value, tokens);
        }

        tokens
    }
}
