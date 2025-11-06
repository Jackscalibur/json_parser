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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
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

fn tokenize_null(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "null".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    Ok(Token::Null)
}

fn make_token(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    let ch = chars[*index];

    let token = match ch {
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => match tokenize_null(chars, index) {
            Ok(token) => token,
            Err(err) => return Err(err),
        },
        't' => todo!("Implement 'true' token"),
        'f' => todo!("Implement 'false' token"),

        _ => todo!("Implement other tokens"),
    };
    Ok(token)
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