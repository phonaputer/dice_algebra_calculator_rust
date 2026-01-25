use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    D,
    KeepHigh,
    KeepLow,
    Add,
    Subtract,
    Multiply,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Integer,
}

pub struct Token {
    pub token_type: TokenType,
    pub integer: u64,
}

#[derive(Debug)]
pub struct LexingError {
    pub message: String,
}

impl LexingError {
    fn new(message: &str) -> LexingError {
        LexingError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl Error for LexingError {}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexingError> {
    let mut ongoing_integer = String::new();
    let mut results: Vec<Token> = Vec::new();

    for char in input.chars() {
        let maybe_token_type: Option<TokenType> = match char {
            'd' | 'D' => Some(TokenType::D),
            'h' | 'H' => Some(TokenType::KeepHigh),
            'l' | 'L' => Some(TokenType::KeepLow),
            '+' => Some(TokenType::Add),
            '-' => Some(TokenType::Subtract),
            '*' => Some(TokenType::Multiply),
            '/' => Some(TokenType::Divide),
            '(' => Some(TokenType::OpenParenthesis),
            ')' => Some(TokenType::CloseParenthesis),
            ' ' | '\n' | '\t' | '\r' => None,
            '0'..='9' => {
                ongoing_integer.push(char);
                None
            }
            _ => {
                return Err(LexingError::new(&format!("Unexpected character: {}", char)));
            }
        };

        let Some(token_type) = maybe_token_type else {
            continue;
        };

        if !ongoing_integer.is_empty() {
            match ongoing_integer.parse::<u64>() {
                Ok(integer) => {
                    results.push(Token {
                        token_type: TokenType::Integer,
                        integer: integer,
                    });
                }
                Err(e) => {
                    return Err(LexingError::new(&format!("Unexpected error: {}", e)));
                }
            };

            ongoing_integer.clear();
        }

        results.push(Token {
            token_type: token_type,
            integer: 0,
        });
    }

    if !ongoing_integer.is_empty() {
        match ongoing_integer.parse::<u64>() {
            Ok(integer) => {
                results.push(Token {
                    token_type: TokenType::Integer,
                    integer: integer,
                });
            }
            Err(e) => {
                return Err(LexingError::new(&format!("Unexpected error: {}", e)));
            }
        };
    }

    return Ok(results);
}
