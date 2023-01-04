use regex::Regex;

// TODO: Convert to Rail Tokens. What's the algorithm anyway?

pub fn tokenize(source: &str) -> Vec<SchemeToken> {
    let re: Regex = Regex::new(r#"(\(|\)|\[|\]|".*?"|[^\s\(\)\[\]]*)"#).unwrap();
    let line = source.replace('\n', " ");
    re.captures_iter(&line)
        .flat_map(|cap| cap.iter().take(1).collect::<Vec<_>>())
        .filter_map(|res| res.map(|mat| mat.as_str().to_owned()))
        .map(SchemeToken::from)
        .collect()
}

#[test]
fn basic_tokens() {
    use SchemeToken::*;

    assert_eq!(
        tokenize("(+ 1 2)"),
        vec![
            LeftParen,
            Identifier("+".into()),
            Identifier("1".into()),
            Identifier("2".into()),
            RightParen
        ]
    );
}

#[test]
fn more_tokens() {
    use SchemeToken::*;

    assert_eq!(
        tokenize("(println (join \"!\" (words \"hey there you\")))"),
        vec![
            LeftParen,
            Identifier("println".into()),
            LeftParen,
            Identifier("join".into()),
            String(",".into()),
            LeftParen,
            Identifier("words".into()),
            String("hey there you".into()),
            RightParen,
            RightParen,
            RightParen
        ]
    );
}

pub enum SchemeValue {
    Static(String),
    Dynamic {
        f: Box<SchemeValue>,
        args: Vec<SchemeValue>,
    },
}

impl From<Vec<SchemeToken>> for SchemeValue {
    fn from(_: Vec<SchemeToken>) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SchemeToken {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Identifier(String),
    String(String),
}

impl From<String> for SchemeToken {
    fn from(token: String) -> Self {
        if token.len() == 1 {
            match token.as_str() {
                "(" => SchemeToken::LeftParen,
                ")" => SchemeToken::RightParen,
                "[" => SchemeToken::LeftBracket,
                "]" => SchemeToken::RightBracket,
                _ => SchemeToken::Identifier(token),
            }
        } else if token.starts_with('"') && token.ends_with('"') {
            let content = token
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .to_string();
            SchemeToken::String(content)
        } else {
            SchemeToken::Identifier(token)
        }
    }
}
