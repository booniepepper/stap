use pest::iterators::Pair;
use pest_derive::Parser;

pub use pest::Parser;

#[derive(Parser)]
#[grammar = "stap.pest"]
pub struct StapParser;

#[derive(Debug, PartialEq, Eq)]
pub struct StapModule {
    pub values: Vec<StapValue>,
}

impl StapModule {
    pub fn parse(source: &str) -> Self {
        let module = StapParser::parse(Rule::module, source)
            .unwrap_or_else(|e| panic!("Error parsing. {:?}", e))
            .next()
            .expect("A module must have content. Did the grammar get mangled?");

        let values = module
            .into_inner()
            .filter_map(|pair| match pair.as_rule() {
                Rule::value => {
                    let inner = pair.into_inner().next().unwrap();
                    Some(StapValue::from(inner))
                }
                Rule::EOI => None,
                rule => unreachable!(
                    "A module can only contain top-level values. Found: {:?}",
                    rule
                ),
            })
            .collect();

        StapModule { values }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum StapValue {
    Function(Vec<StapValue>),
    List(Vec<StapValue>),
    String(String),
    Identifier(String),
}

impl From<Pair<'_, Rule>> for StapValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::function => Self::Function(pair.into_inner().map(StapValue::from).collect()),
            Rule::list => Self::List(pair.into_inner().map(StapValue::from).collect()),
            Rule::quoted_string => StapValue::from(pair.into_inner().next().unwrap()),
            Rule::string => Self::String(pair.as_str().to_owned()),
            Rule::identifier => Self::Identifier(pair.as_str().to_owned()),
            Rule::value => StapValue::from(pair.into_inner().next().unwrap()),
            _ => unreachable!(
                "A StapValue must be a function, list, string, or identifier. Found: {:?}",
                pair
            ),
        }
    }
}

/* --- All tests from here --- */

#[test]
fn parse_a_module_i_guess() {
    use StapValue::*;

    let module = StapModule::parse("(println \"look ma I did it\")");

    assert_eq!(
        module,
        StapModule {
            values: vec![Function(vec![
                Identifier("println".into()),
                String("look ma I did it".into())
            ])]
        }
    );
}

#[allow(dead_code)]
fn test_parse(rule: Rule, input: &str) -> Pair<'_, Rule> {
    StapParser::parse(rule, input)
        .expect("parse failed")
        .next()
        .unwrap()
}

#[test]
fn simple_module() {
    let res = test_parse(Rule::module, "(println (+ 1 2))");

    // I.e. it should have nothing omitted.
    assert_eq!(res.as_str(), "(println (+ 1 2))");
}

#[test]
fn empty_module() {
    let res = test_parse(Rule::module, "");

    assert_eq!(res.as_str(), "");
}
