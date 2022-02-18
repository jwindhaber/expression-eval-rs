use crate::definition::Operator;
use crate::Literal;

use alloc::boxed::Box;

pub struct OperatorExecutor {
    operator_command_factory: OperatorCommandFactory,
}

impl OperatorExecutor {
    pub fn new() -> Self {
        Self { operator_command_factory: OperatorCommandFactory {} }
    }

    pub fn execute(&self, operator: &Operator, right_side: &Literal, left_side: &Literal) -> Result<Literal, &'static str> {
        self.operator_command_factory.get_command(operator).execute_command(right_side, left_side)
    }
}


pub struct OperatorCommandFactory {}



impl OperatorCommandFactory {
    pub fn get_command(&self, operator: &Operator) -> &dyn OperatorCommand {
        match operator {
            Operator::Or => { &OrCommand {} }
            Operator::And => { &AndCommand {} }
            Operator::Not => { &NotCommand {} }
            Operator::NotEqual => { &NotEqualCommand {} }
            Operator::Equal => { &EqualCommand {} }
            Operator::Greater => { &GreaterCommand {} }
            Operator::GreaterOrEqual => { &GreaterOrEqualCommand {} }
            Operator::Less => { &LessCommand {} }
            Operator::LessOrEqual => { &LessOrEqualCommand {} }
            Operator::Plus => { &PlusCommand {} }
            Operator::Minus => { &MinusCommand {} }
            Operator::Divide => { &DivideCommand {} }
            Operator::Multiply => { &MultiplyCommand {} }
            Operator::PowerOf => { &PowerOfCommand {} }
        }
    }
}

pub trait OperatorCommand {
    fn execute_command(&self, right_side: &Literal, left_side: &Literal) -> Result<Literal, &'static str> {
        if let (Literal::Integer(left), Literal::Integer(right)) = (&left_side, &right_side) {
            self.eval_integer_literals(*left, *right)
        } else if let (Literal::Decimal(left), Literal::Decimal(right)) = (&left_side, &right_side) {
            // both are decimals
            self.eval_decimal_literals(*left, *right)
        } else if let (Literal::Decimal(left), Literal::Integer(right_integer)) = (&left_side, &right_side) {
            // different transform to decimal
            let right = *right_integer as f64;
            self.eval_decimal_literals(*left, right)
        } else if let (Literal::Integer(left_integer), Literal::Decimal(right)) = (&left_side, &right_side) {
            // different transform to decimal
            let left = *left_integer as f64;
            self.eval_decimal_literals(left, *right)
        } else if let (Literal::Boolean(left), Literal::Boolean(right)) = (&left_side, &right_side) {
            self.eval_boolean_literals(*left, *right)
        } else if let (Literal::String(left), Literal::String(right)) = (&left_side, &right_side) {
            // both are strings
            self.eval_string_literals(&left, &right)
        } else {
            Err("Either the left or the right literal is not supported")
        }
    }

    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str>;

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str>;

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str>;

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str>;
}

struct OrCommand {}

impl OperatorCommand for OrCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left || right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Err("<left || right> is not applicable for integers!")
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Err("<left || right> is not applicable for integers!")
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left || right> is not applicable for integers!")
    }
}

struct AndCommand {}

impl OperatorCommand for AndCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left && right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Err("<left && right> is not applicable for integers!")
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Err("<left && right> is not applicable for decimals!")
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left && right> is not applicable for strings!")
    }
}

struct NotCommand {}

impl OperatorCommand for NotCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Err("<!> is not applicable for two booleans!")
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Err("<!> is not applicable for two integers!")
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Err("<!> is not applicable for two decimals!")
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<!> is not applicable for two strings!")
    }
}

struct NotEqualCommand {}

impl OperatorCommand for NotEqualCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left != right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left != right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left != right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left != right))
    }
}

struct EqualCommand {}

impl OperatorCommand for EqualCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left == right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left == right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left == right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left == right))
    }
}

struct GreaterCommand {}

impl OperatorCommand for GreaterCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left > right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left > right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left > right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left > right))
    }
}

struct GreaterOrEqualCommand {}

impl OperatorCommand for GreaterOrEqualCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left >= right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left >= right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left >= right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left >= right))
    }
}

struct LessCommand {}

impl OperatorCommand for LessCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left < right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left < right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left < right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left < right))
    }
}

struct LessOrEqualCommand {}

impl OperatorCommand for LessOrEqualCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left <= right))
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left <= right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left <= right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Ok(Literal::Boolean(left <= right))
    }
}

struct PlusCommand {}

impl OperatorCommand for PlusCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Err("<left + right> is not applicable for booleans!")
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Integer(left + right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Decimal(left + right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left + right> is not applicable for two strings!")
        //TODO check if supported
        // Ok(Literal::String(Box::from(format!("{}{}", left, right))))
    }
}

struct MinusCommand {}

impl OperatorCommand for MinusCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Err("<left - right> is not applicable for booleans!")
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Integer(left - right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Decimal(left - right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left - right> is not applicable for two strings!")
    }
}

struct DivideCommand {}

impl OperatorCommand for DivideCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Err("<left / right> is not applicable for booleans!")
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        //TODO do we always want an integer or better transform to decimal???
        Ok(Literal::Integer(left / right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Decimal(left / right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left / right > is not applicable for two strings!")
    }
}

struct MultiplyCommand {}

impl OperatorCommand for MultiplyCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Err("<left * right> is not applicable for booleans!")
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Integer(left * right))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Ok(Literal::Decimal(left * right))
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left * right> is not applicable for two strings!")
    }
}

struct PowerOfCommand {}

impl OperatorCommand for PowerOfCommand {
    fn eval_boolean_literals(&self, left: bool, right: bool) -> Result<Literal, &'static str> {
        Err("<left ^ right> is not applicable for booleans!")
    }

    fn eval_integer_literals(&self, left: i64, right: i64) -> Result<Literal, &'static str> {
        Ok(Literal::Integer(left.pow(right as u32)))
    }

    fn eval_decimal_literals(&self, left: f64, right: f64) -> Result<Literal, &'static str> {
        Err("<left ^ right> is not applicable for decimals!")
    }

    fn eval_string_literals(&self, left: &str, right: &str) -> Result<Literal, &'static str> {
        Err("<left ^ right> is not applicable for two strings!")
    }
}

