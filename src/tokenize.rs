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
        let token = make_token(chars[index]);
        tokens.push(token);
    }
    tokens
}

fn make_token(ch: char) -> Token {
    match ch {
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => todo!("Implement 'null' token"),
        't' => todo!("Implement 'true' token"),
        'f' => todo!("Implement 'false' token"),

        _ => todo!("Implement other tokens"),
    }
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