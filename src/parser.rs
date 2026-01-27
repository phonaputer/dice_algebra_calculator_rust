use crate::ast::AST;
use crate::ast::IntegerAST;
use crate::ast::LongRollAST;
use crate::ast::MathAST;
use crate::ast::MathOperation;
use crate::ast::ShortRollAST;
use crate::dice_error::DiceError;
use crate::lexer::Token;
use crate::lexer::TokenType;

struct TokenItr<'a> {
    tokens: &'a [Token],
    cur_token: usize,
}

impl TokenItr<'_> {
    fn next(&mut self) -> Option<&Token> {
        if self.cur_token >= self.tokens.len() {
            return None;
        }

        let result = &self.tokens[self.cur_token];

        self.cur_token += 1;

        return Some(result);
    }

    fn peek(&self) -> Option<&Token> {
        if self.cur_token >= self.tokens.len() {
            return None;
        }

        return Some(&self.tokens[self.cur_token]);
    }

    fn peek_next(&self) -> Option<&Token> {
        let peek_idx = self.cur_token + 1;

        if peek_idx >= self.tokens.len() {
            return None;
        }

        return Some(&self.tokens[peek_idx]);
    }

    fn is_empty(&self) -> bool {
        return self.cur_token >= self.tokens.len();
    }
}

pub fn parse(tokens: &[Token]) -> Result<AST, DiceError> {
    validate_not_empty(tokens)?;
    validate_parenthesis(tokens)?;

    let mut itr = TokenItr {
        tokens: tokens,
        cur_token: 0,
    };

    let result = parse_add(&mut itr);

    validate_consumed_all_tokens(&itr)?;

    return result;
}

fn parse_add(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    let mut left = parse_mult(tokens)?;

    while let Some(token) = tokens.peek() {
        let operation = match token.token_type {
            TokenType::Add => MathOperation::Add,
            TokenType::Subtract => MathOperation::Subtract,
            _ => return Ok(left),
        };

        tokens.next(); // discard + or - token

        let right = parse_mult(tokens)?;

        let new_left = AST::Math(MathAST {
            left: Box::new(left),
            right: Box::new(right),
            operation: operation,
        });

        left = new_left;
    }

    return Ok(left);
}

fn parse_mult(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    let mut left = parse_atom(tokens)?;

    while let Some(token) = tokens.peek() {
        let operation = match token.token_type {
            TokenType::Multiply => MathOperation::Multiply,
            TokenType::Divide => MathOperation::Divide,
            _ => return Ok(left),
        };

        tokens.next(); // discard * or / token

        let right = parse_atom(tokens)?;

        let new_left = AST::Math(MathAST {
            left: Box::new(left),
            right: Box::new(right),
            operation: operation,
        });

        left = new_left;
    }

    return Ok(left);
}

fn parse_atom(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    let next_token = match tokens.peek() {
        None => return Err(DiceError::new("Invalid expression.")),
        Some(token) => token,
    };

    if next_token.token_type != TokenType::OpenParenthesis {
        return parse_roll(tokens);
    }

    tokens.next(); // discard ( token
    let result = parse_add(tokens);
    tokens.next(); // discard ) token

    return result;
}

fn parse_roll(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    let next_token = match tokens.peek() {
        None => return Err(DiceError::new("Invalid expression.")),
        Some(token) => token,
    };

    if next_token.token_type == TokenType::D {
        return parse_shortroll(tokens);
    }

    if let Some(token) = tokens.peek_next()
        && token.token_type == TokenType::D
    {
        return parse_longroll(tokens);
    }

    return parse_integer(tokens);
}

fn parse_longroll(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    let die = parse_integer_raw(tokens)?;

    match tokens.next() {
        None => return Err(DiceError::new("Invalid expression.")),
        Some(token) => {
            if token.token_type != TokenType::D {
                return Err(DiceError::new("Invalid expression"));
            }
        }
    };

    let faces = parse_integer_raw(tokens)?;

    let mut result_data = LongRollAST {
        die: die,
        faces: faces,
        keep_high: None,
        keep_low: None,
    };

    let maybe_token = tokens.peek();
    if let Some(token) = maybe_token {
        match token.token_type {
            TokenType::KeepHigh => {
                tokens.next(); // discard h token
                result_data.keep_high = Some(parse_integer_raw(tokens)?)
            }
            TokenType::KeepLow => {
                tokens.next(); // discard l token
                result_data.keep_low = Some(parse_integer_raw(tokens)?)
            }
            _ => {
                // do nothing
            }
        }
    }

    return Ok(AST::LongRoll(result_data));
}

fn parse_shortroll(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    if let Some(token) = tokens.next()
        && token.token_type == TokenType::D
    {
        return Ok(AST::ShortRoll(ShortRollAST {
            faces: parse_integer_raw(tokens)?,
        }));
    }

    return Err(DiceError::new(
        "Parse shortroll should not be called when the next token is not D.",
    ));
}

fn parse_integer(tokens: &mut TokenItr) -> Result<AST, DiceError> {
    return Ok(AST::Integer(IntegerAST {
        integer: parse_integer_raw(tokens)?,
    }));
}

fn parse_integer_raw(tokens: &mut TokenItr) -> Result<u64, DiceError> {
    if let Some(token) = tokens.next()
        && token.token_type == TokenType::Integer
    {
        return Ok(token.integer);
    }

    return Err(DiceError::new("Invalid expression."));
}

fn validate_consumed_all_tokens(tokens: &TokenItr) -> Result<(), DiceError> {
    if !tokens.is_empty() {
        return Err(DiceError::new("Invalid expression."));
    }

    return Ok(());
}

fn validate_not_empty(tokens: &[Token]) -> Result<(), DiceError> {
    if tokens.len() < 1 {
        return Err(DiceError::new("Invalid expression."));
    }

    return Ok(());
}

fn validate_parenthesis(tokens: &[Token]) -> Result<(), DiceError> {
    let mut open_count = 0;
    let mut close_count = 0;

    for token in tokens {
        if token.token_type == TokenType::OpenParenthesis {
            open_count += 1;
        }
        if token.token_type == TokenType::CloseParenthesis {
            close_count += 1;
        }
    }

    if open_count != close_count {
        return Err(DiceError::new(
            "Expression contains an unclosed parenthetical.",
        ));
    }

    return Ok(());
}
