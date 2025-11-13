use std::collections::HashMap;

/// The main JSON value type representing any valid JSON value
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    /// Parse a JSON string into a Value
    pub fn from_str(input: &str) -> Result<Self, String> {
        Parser::new(input).parse()
    }
}

/// Simple JSON parser
struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        let value = self.parse_value()?;
        self.skip_whitespace();
        
        if self.pos < self.chars.len() {
            return Err(format!("Unexpected characters after JSON value"));
        }
        
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_whitespace();
        
        if self.pos >= self.chars.len() {
            return Err("Unexpected end of input".to_string());
        }

        match self.chars[self.pos] {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_boolean(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            c => Err(format!("Unexpected character: '{}'", c)),
        }
    }

    fn parse_null(&mut self) -> Result<Value, String> {
        if self.consume_literal("null") {
            Ok(Value::Null)
        } else {
            Err("Invalid null literal".to_string())
        }
    }

    fn parse_boolean(&mut self) -> Result<Value, String> {
        if self.consume_literal("true") {
            Ok(Value::Boolean(true))
        } else if self.consume_literal("false") {
            Ok(Value::Boolean(false))
        } else {
            Err("Invalid boolean literal".to_string())
        }
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        
        // Optional minus
        if self.peek() == Some('-') {
            self.pos += 1;
        }

        // Integer part
        if self.peek() == Some('0') {
            self.pos += 1;
        } else if self.peek().map_or(false, |c| c.is_ascii_digit()) {
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.pos += 1;
            }
        } else {
            return Err("Invalid number".to_string());
        }

        // Optional fractional part
        if self.peek() == Some('.') {
            self.pos += 1;
            if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
                return Err("Invalid number: decimal point must be followed by digit".to_string());
            }
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        // Optional exponent
        if self.peek() == Some('e') || self.peek() == Some('E') {
            self.pos += 1;
            if self.peek() == Some('+') || self.peek() == Some('-') {
                self.pos += 1;
            }
            if !self.peek().map_or(false, |c| c.is_ascii_digit()) {
                return Err("Invalid number: exponent must have digits".to_string());
            }
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        let num_str: String = self.chars[start..self.pos].iter().collect();
        num_str
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| "Failed to parse number".to_string())
    }

    fn parse_string(&mut self) -> Result<Value, String> {
        self.pos += 1; // consume opening quote
        let mut result = String::new();

        while self.pos < self.chars.len() {
            match self.chars[self.pos] {
                '"' => {
                    self.pos += 1;
                    return Ok(Value::String(result));
                }
                '\\' => {
                    self.pos += 1;
                    if self.pos >= self.chars.len() {
                        return Err("Unterminated string escape".to_string());
                    }
                    match self.chars[self.pos] {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\u{0008}'),
                        'f' => result.push('\u{000C}'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'u' => {
                            self.pos += 1;
                            if self.pos + 4 > self.chars.len() {
                                return Err("Invalid unicode escape".to_string());
                            }
                            let hex: String = self.chars[self.pos..self.pos + 4].iter().collect();
                            let code = u32::from_str_radix(&hex, 16)
                                .map_err(|_| "Invalid unicode escape")?;
                            let ch = char::from_u32(code)
                                .ok_or_else(|| "Invalid unicode code point")?;
                            result.push(ch);
                            self.pos += 3; // Will be incremented by 1 at end of loop
                        }
                        c => return Err(format!("Invalid escape sequence: \\{}", c)),
                    }
                    self.pos += 1;
                }
                c => {
                    result.push(c);
                    self.pos += 1;
                }
            }
        }

        Err("Unterminated string".to_string())
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.pos += 1; // consume '['
        let mut elements = Vec::new();

        self.skip_whitespace();
        
        // Empty array
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(Value::Array(elements));
        }

        loop {
            elements.push(self.parse_value()?);
            self.skip_whitespace();

            match self.peek() {
                Some(',') => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                Some(']') => {
                    self.pos += 1;
                    return Ok(Value::Array(elements));
                }
                _ => return Err("Expected ',' or ']' in array".to_string()),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        self.pos += 1; // consume '{'
        let mut object = HashMap::new();

        self.skip_whitespace();

        // Empty object
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(Value::Object(object));
        }

        loop {
            self.skip_whitespace();

            // Parse key (must be a string)
            if self.peek() != Some('"') {
                return Err("Expected string key in object".to_string());
            }

            let key = match self.parse_string()? {
                Value::String(s) => s,
                _ => unreachable!(),
            };

            self.skip_whitespace();

            // Expect colon
            if self.peek() != Some(':') {
                return Err("Expected ':' after object key".to_string());
            }
            self.pos += 1;

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();

            match self.peek() {
                Some(',') => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                Some('}') => {
                    self.pos += 1;
                    return Ok(Value::Object(object));
                }
                _ => return Err("Expected ',' or '}' in object".to_string()),
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.chars.len() {
            Some(self.chars[self.pos])
        } else {
            None
        }
    }

    fn consume_literal(&mut self, literal: &str) -> bool {
        let chars: Vec<char> = literal.chars().collect();
        
        if self.pos + chars.len() > self.chars.len() {
            return false;
        }

        for (i, &ch) in chars.iter().enumerate() {
            if self.chars[self.pos + i] != ch {
                return false;
            }
        }

        self.pos += chars.len();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        assert_eq!(Value::from_str("null").unwrap(), Value::Null);
    }

    #[test]
    fn test_boolean() {
        assert_eq!(Value::from_str("true").unwrap(), Value::Boolean(true));
        assert_eq!(Value::from_str("false").unwrap(), Value::Boolean(false));
    }

    #[test]
    fn test_number() {
        assert_eq!(Value::from_str("42").unwrap(), Value::Number(42.0));
        assert_eq!(Value::from_str("-17").unwrap(), Value::Number(-17.0));
        assert_eq!(Value::from_str("3.14").unwrap(), Value::Number(3.14));
        assert_eq!(Value::from_str("1e10").unwrap(), Value::Number(1e10));
        assert_eq!(Value::from_str("2.5e-3").unwrap(), Value::Number(2.5e-3));
    }

    #[test]
    fn test_string() {
        assert_eq!(
            Value::from_str(r#""hello""#).unwrap(),
            Value::String("hello".to_string())
        );
        assert_eq!(
            Value::from_str(r#""hello\nworld""#).unwrap(),
            Value::String("hello\nworld".to_string())
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(Value::from_str("[]").unwrap(), Value::Array(vec![]));
        assert_eq!(
            Value::from_str("[1, 2, 3]").unwrap(),
            Value::Array(vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ])
        );
    }

    #[test]
    fn test_object() {
        let result = Value::from_str(r#"{"name": "John", "age": 30}"#).unwrap();
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), Value::String("John".to_string()));
        expected.insert("age".to_string(), Value::Number(30.0));
        assert_eq!(result, Value::Object(expected));
    }

    #[test]
    fn test_nested() {
        let json = r#"{
            "name": "Alice",
            "age": 30,
            "active": true,
            "scores": [95, 87, 92],
            "address": {
                "city": "NYC",
                "zip": "10001"
            }
        }"#;
        
        let result = Value::from_str(json).unwrap();
        
        if let Value::Object(obj) = result {
            assert_eq!(obj.get("name"), Some(&Value::String("Alice".to_string())));
            assert_eq!(obj.get("age"), Some(&Value::Number(30.0)));
            assert_eq!(obj.get("active"), Some(&Value::Boolean(true)));
        } else {
            panic!("Expected object");
        }
    }
}
