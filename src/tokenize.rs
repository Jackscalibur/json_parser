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
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;

    let mut tokens = Vec::new();
    while index < chars.len() {
        index += 1;
    }
    tokens
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