mod parser;
pub use parser::*;
mod prompt;
pub use prompt::*;

use rail_lang::{rail_machine::RailState, tokens::Token};

pub fn run(state: RailState, module: Module) -> RailState {
    let rail_tokens = module
        .values
        .into_iter()
        .flat_map(to_rail_tokens)
        .collect::<Vec<_>>();

    state.run_tokens(rail_tokens)
}

fn to_rail_tokens(value: Value) -> Vec<Token> {
    match value {
        Value::String(s) => vec![Token::String(s)],
        Value::Identifier(i) => vec![Token::from(i)],
        Value::List(vals) => vals.into_iter().flat_map(to_rail_tokens).rev().collect(),
        Value::Function(args) => args.into_iter().flat_map(to_rail_tokens).rev().collect(),
    }
}
