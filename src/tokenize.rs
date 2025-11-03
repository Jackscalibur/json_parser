#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Null,
    True,
    False,
    Number(f64),
    String(String),
}

pub fn tokenize(input: String) -> Vec<Token> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token};

    #[test]
    fn just_comma() {
        let input = String::from(",");
        let expected = [Token::Comma];

        let actual = tokenize(input);

        assert_eq!(actual, expected);
    }
}