use crate::dice_error::DiceError;

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub integer: u64,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, DiceError> {
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
                return Err(DiceError::new(&format!("Unexpected character: {}", char)));
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
                    return Err(DiceError::new(&format!("Unexpected error: {}", e)));
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
                return Err(DiceError::new(&format!("Unexpected error: {}", e)));
            }
        };
    }

    return Ok(results);
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_emptyInput_returnsEmptyVector() {
        let input = "";

        let result = tokenize(input).unwrap();

        assert_eq!(0, result.len());
    }

    #[test]
    fn tokenize_inputOnlyWhitespace_returnsEmptyVector() {
        let input = " \n\r\t";

        let result = tokenize(input).unwrap();

        assert_eq!(0, result.len());
    }

    #[test]
    fn tokenize_inputUnexpectedCharacter_returnsDiceError() {
        let input = "k";

        match tokenize(input) {
            Err(err) => {
                assert_eq!("Unexpected character: k", err.message)
            }
            Ok(_) => assert!(false, "Should have returned an error"),
        }
    }

    #[test]
    fn tokenize_inputValidCharacters_returnsMatchingTokensForEach() {
        let input = "100dD1+-*/()lLhH";

        let result = tokenize(input).unwrap();

        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::Integer,
                integer: 100,
            },
            Token {
                token_type: TokenType::D,
                integer: 0,
            },
            Token {
                token_type: TokenType::D,
                integer: 0,
            },
            Token {
                token_type: TokenType::Integer,
                integer: 1,
            },
            Token {
                token_type: TokenType::Add,
                integer: 0,
            },
            Token {
                token_type: TokenType::Subtract,
                integer: 0,
            },
            Token {
                token_type: TokenType::Multiply,
                integer: 0,
            },
            Token {
                token_type: TokenType::Divide,
                integer: 0,
            },
            Token {
                token_type: TokenType::OpenParenthesis,
                integer: 0,
            },
            Token {
                token_type: TokenType::CloseParenthesis,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepLow,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepLow,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepHigh,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepHigh,
                integer: 0,
            },
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn tokenize_inputValidCharactersWithWhitespace_returnsMatchingTokensIgnoringWhitespace() {
        let input = "100 \n\r\td \n\r\tD \n\r\t1 \n\r\t+ \n\r\t- \n\r\t* \n\r\t/ \n\r\t( \n\r\t) \n\r\tl \n\r\tL \n\r\th \n\r\tH";

        let result = tokenize(input).unwrap();

        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::Integer,
                integer: 100,
            },
            Token {
                token_type: TokenType::D,
                integer: 0,
            },
            Token {
                token_type: TokenType::D,
                integer: 0,
            },
            Token {
                token_type: TokenType::Integer,
                integer: 1,
            },
            Token {
                token_type: TokenType::Add,
                integer: 0,
            },
            Token {
                token_type: TokenType::Subtract,
                integer: 0,
            },
            Token {
                token_type: TokenType::Multiply,
                integer: 0,
            },
            Token {
                token_type: TokenType::Divide,
                integer: 0,
            },
            Token {
                token_type: TokenType::OpenParenthesis,
                integer: 0,
            },
            Token {
                token_type: TokenType::CloseParenthesis,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepLow,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepLow,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepHigh,
                integer: 0,
            },
            Token {
                token_type: TokenType::KeepHigh,
                integer: 0,
            },
        ];
        assert_eq!(expected, result);
    }
}
