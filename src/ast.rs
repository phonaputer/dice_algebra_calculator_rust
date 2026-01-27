use crate::dice_error::DiceError;
use rand;

pub struct IntegerAST {
    pub integer: u64,
}

pub struct ShortRollAST {
    pub faces: u64,
}

pub struct LongRollAST {
    pub die: u64,
    pub faces: u64,
    pub keep_high: Option<u64>,
    pub keep_low: Option<u64>,
}

pub enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct MathAST {
    pub operation: MathOperation,
    pub left: Box<AST>,
    pub right: Box<AST>,
}

pub enum AST {
    Integer(IntegerAST),
    ShortRoll(ShortRollAST),
    LongRoll(LongRollAST),
    Math(MathAST),
}

pub struct ASTExecutionResult {
    pub result: i128,
    pub description: String,
}

pub trait ASTExecutable {
    fn execute_ast(&self, rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError>;
}

impl ASTExecutable for AST {
    fn execute_ast(&self, rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError> {
        match self {
            AST::Integer(ast) => ast.execute_ast(rng),
            AST::ShortRoll(ast) => ast.execute_ast(rng),
            AST::LongRoll(ast) => ast.execute_ast(rng),
            AST::Math(ast) => ast.execute_ast(rng),
        }
    }
}

impl ASTExecutable for IntegerAST {
    fn execute_ast(&self, _rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError> {
        return Ok(ASTExecutionResult {
            result: self.integer as i128,
            description: String::new(),
        });
    }
}

impl ASTExecutable for ShortRollAST {
    fn execute_ast(&self, rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError> {
        let roll = rng.random_range(1..self.faces);

        return Ok(ASTExecutionResult {
            result: roll as i128,
            description: format!("\nRolling d{}...\nYou rolled: {}", self.faces, roll),
        });
    }
}

impl ASTExecutable for LongRollAST {
    fn execute_ast(&self, rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError> {
        let mut description = format!("\nRolling {}d{}...\n", self.die, self.faces);
        let mut rolls: Vec<u64> = Vec::new();
        let mut sum: i128 = 0;

        for _ in 0..self.die {
            let roll = rng.random_range(1..self.faces);

            rolls.push(roll);
            description.push_str(&format!("You rolled: {}\n", roll));
            sum = sum + roll as i128;
        }

        if let Some(keep_low) = self.keep_low {
            if keep_low < self.die {
                rolls.sort_unstable();
                sum = 0;

                for &roll in rolls.iter().take(keep_low as usize) {
                    sum = sum + roll as i128;
                }
            }
        } else if let Some(keep_high) = self.keep_high {
            if keep_high < self.die {
                rolls.sort_unstable();
                sum = 0;

                for &roll in rolls.iter().rev().take(keep_high as usize) {
                    sum = sum + roll as i128;
                }
            }
        }

        return Ok(ASTExecutionResult {
            result: sum,
            description: description,
        });
    }
}

impl ASTExecutable for MathAST {
    fn execute_ast(&self, rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError> {
        let left = self.left.execute_ast(rng)?;
        let right = self.left.execute_ast(rng)?;

        let result = match self.operation {
            MathOperation::Add => left.result + right.result,
            MathOperation::Subtract => left.result - right.result,
            MathOperation::Multiply => left.result * right.result,
            MathOperation::Divide => {
                if right.result == 0 {
                    return Err(DiceError::new("Division by zero is not allowed."));
                }

                left.result / right.result
            }
        };

        return Ok(ASTExecutionResult {
            result: result,
            description: format!("{}\n{}", left.description, right.description),
        });
    }
}
