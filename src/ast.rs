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
        let roll = match self.faces {
            0 => 0,
            1 => 1,
            _ => rng.random_range(1..self.faces),
        };

        return Ok(ASTExecutionResult {
            result: roll as i128,
            description: format!("\nRolling d{}...\nYou rolled: {}\n", self.faces, roll),
        });
    }
}

impl ASTExecutable for LongRollAST {
    fn execute_ast(&self, rng: &mut impl rand::Rng) -> Result<ASTExecutionResult, DiceError> {
        let mut description = format!("\nRolling {}d{}...\n", self.die, self.faces);
        let mut rolls: Vec<u64> = Vec::new();
        let mut sum: i128 = 0;

        for _ in 0..self.die {
            let roll = match self.faces {
                0 => 0,
                1 => 1,
                _ => rng.random_range(1..self.faces),
            };

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
        let right = self.right.execute_ast(rng)?;

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
            description: format!("{}{}", left.description, right.description),
        });
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn IntegerAST__execute_ast__with_static_value__just_returns_the_value() {
        let ast = AST::Integer(IntegerAST { integer: 10 });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(10 as i128, result.result);
        assert_eq!("", result.description);
    }

    #[test]
    fn ShortRollAST__execute_ast__N_faces__rolls_one_die_with_N_faces() {
        let ast = AST::ShortRoll(ShortRollAST { faces: 10 });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(9 as i128, result.result);
        assert_eq!("\nRolling d10...\nYou rolled: 9\n", result.description);
    }

    #[test]
    fn ShortRollAST__execute_ast__0_faces__rolls_one_die_with_0_faces() {
        let ast = AST::ShortRoll(ShortRollAST { faces: 0 });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(0 as i128, result.result);
        assert_eq!("\nRolling d0...\nYou rolled: 0\n", result.description);
    }

    #[test]
    fn ShortRollAST__execute_ast__1_face__rolls_one_die_with_1_face() {
        let ast = AST::ShortRoll(ShortRollAST { faces: 1 });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(1 as i128, result.result);
        assert_eq!("\nRolling d1...\nYou rolled: 1\n", result.description);
    }

    #[test]
    fn LongRollAST__execute_ast__N_die_M_faces__rolls_N_die_with_M_faces() {
        let ast = AST::LongRoll(LongRollAST {
            die: 2,
            faces: 10,
            keep_high: None,
            keep_low: None,
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(16, result.result);
        assert_eq!(
            "\nRolling 2d10...\nYou rolled: 9\nYou rolled: 7\n",
            result.description
        );
    }

    #[test]
    fn LongRollAST__execute_ast__0_die_M_faces__rolls_0_die() {
        let ast = AST::LongRoll(LongRollAST {
            die: 0,
            faces: 10,
            keep_high: None,
            keep_low: None,
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(0, result.result);
        assert_eq!("\nRolling 0d10...\n", result.description);
    }

    #[test]
    fn LongRollAST__execute_ast__N_die_0_faces__rolls_N_die_with_0_faces() {
        let ast = AST::LongRoll(LongRollAST {
            die: 2,
            faces: 0,
            keep_high: None,
            keep_low: None,
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(0, result.result);
        assert_eq!(
            "\nRolling 2d0...\nYou rolled: 0\nYou rolled: 0\n",
            result.description
        );
    }

    #[test]
    fn LongRollAST__execute_ast__keep_high_N__keeps_only_the_highest_N_rolls() {
        let ast = AST::LongRoll(LongRollAST {
            die: 2,
            faces: 10,
            keep_high: Some(1),
            keep_low: None,
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(9, result.result);
        assert_eq!(
            "\nRolling 2d10...\nYou rolled: 9\nYou rolled: 7\n",
            result.description
        );
    }

    #[test]
    fn LongRollAST__execute_ast__keep_low_N__keeps_only_the_lowest_N_rolls() {
        let ast = AST::LongRoll(LongRollAST {
            die: 2,
            faces: 10,
            keep_high: None,
            keep_low: Some(1),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(7, result.result);
        assert_eq!(
            "\nRolling 2d10...\nYou rolled: 9\nYou rolled: 7\n",
            result.description
        );
    }

    #[test]
    fn LongRollAST__execute_ast__keep_low_N_keep_high_M__keeps_only_the_lowest_N_rolls() {
        let ast = AST::LongRoll(LongRollAST {
            die: 2,
            faces: 10,
            keep_high: Some(1),
            keep_low: Some(1),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(7, result.result);
        assert_eq!(
            "\nRolling 2d10...\nYou rolled: 9\nYou rolled: 7\n",
            result.description
        );
    }

    #[test]
    fn MathAST__execute_ast__add_operation__adds_left_to_right() {
        let left = AST::Integer(IntegerAST { integer: 10 });
        let right = AST::Integer(IntegerAST { integer: 2 });
        let ast = AST::Math(MathAST {
            operation: MathOperation::Add,
            left: Box::new(left),
            right: Box::new(right),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(12 as i128, result.result);
        assert_eq!("", result.description);
    }

    #[test]
    fn MathAST__execute_ast__subtract_operation__subtracts_left_from_right() {
        let left = AST::Integer(IntegerAST { integer: 10 });
        let right = AST::Integer(IntegerAST { integer: 2 });
        let ast = AST::Math(MathAST {
            operation: MathOperation::Subtract,
            left: Box::new(left),
            right: Box::new(right),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(8 as i128, result.result);
        assert_eq!("", result.description);
    }

    #[test]
    fn MathAST__execute_ast__multiply_operation__multiplies_left_with_right() {
        let left = AST::Integer(IntegerAST { integer: 10 });
        let right = AST::Integer(IntegerAST { integer: 2 });
        let ast = AST::Math(MathAST {
            operation: MathOperation::Multiply,
            left: Box::new(left),
            right: Box::new(right),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(20 as i128, result.result);
        assert_eq!("", result.description);
    }

    #[test]
    fn MathAST__execute_ast__divide_operation__divides_left_by_right() {
        let left = AST::Integer(IntegerAST { integer: 10 });
        let right = AST::Integer(IntegerAST { integer: 2 });
        let ast = AST::Math(MathAST {
            operation: MathOperation::Divide,
            left: Box::new(left),
            right: Box::new(right),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(5 as i128, result.result);
        assert_eq!("", result.description);
    }

    #[test]
    fn MathAST__execute_ast__division_by_zero__returns_DiceError() {
        let left = AST::Integer(IntegerAST { integer: 10 });
        let right = AST::Integer(IntegerAST { integer: 0 });
        let ast = AST::Math(MathAST {
            operation: MathOperation::Divide,
            left: Box::new(left),
            right: Box::new(right),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        match ast.execute_ast(&mut rng) {
            Err(err) => {
                assert_eq!("Division by zero is not allowed.", err.message)
            }
            Ok(_) => assert!(false, "Should have returned an error"),
        }
    }

    #[test]
    fn MathAST__execute_ast__left_and_right_have_descriptions__the_descriptions_are_concatenated() {
        let left = AST::ShortRoll(ShortRollAST { faces: (10) });
        let right = AST::ShortRoll(ShortRollAST { faces: (2) });
        let ast = AST::Math(MathAST {
            operation: MathOperation::Add,
            left: Box::new(left),
            right: Box::new(right),
        });
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);

        let result = ast.execute_ast(&mut rng).unwrap();

        assert_eq!(10 as i128, result.result);
        assert_eq!(
            "\nRolling d10...\nYou rolled: 9\n\nRolling d2...\nYou rolled: 1\n",
            result.description
        );
    }
}
