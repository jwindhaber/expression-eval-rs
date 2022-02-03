use crate::definition::Operator;
use crate::Literal;



/// this function evaluates all operators with boolean input
pub fn eval_boolean_literals(operator: Operator, left: bool, right: bool) -> Result<Literal, &'static str> {

    match operator {
        Operator::Or => {
            Ok(Literal::Boolean(left || right))
        }
        Operator::And => {
            Ok(Literal::Boolean(left && right))
        }
        Operator::Not => {
            Err("<!> is not applicable for two booleans!")
        }
        Operator::NotEqual => {
            Ok(Literal::Boolean(left != right))
        }
        Operator::Equal => {
            Ok(Literal::Boolean(left == right))
        }
        Operator::Greater => {
            Ok(Literal::Boolean(left > right))
        }
        Operator::GreaterOrEqual => {
            Ok(Literal::Boolean(left >= right))
        }
        Operator::Less => {
            Ok(Literal::Boolean(left < right))
        }
        Operator::LessOrEqual => {
            Ok(Literal::Boolean(left <= right))
        }
        Operator::Plus => {
            Err("<left + right> is not applicable for booleans!")
        }
        Operator::Minus => {
             Err("<left - right> is not applicable for booleans!")
        }
        Operator::Divide => {
            Err("<left / right> is not applicable for booleans!")
        }
        Operator::Multiply => {
            Err("<left * right> is not applicable for booleans!")
        }
        Operator::PowerOf => {
            Err("<left^right> is not applicable for booleans!")
        }
    }
}


/// this function evaluates all operators with integer input
pub(crate) fn eval_integer_literals(operator: Operator, left: i64, right: i64) -> Result<Literal, &'static str> {

    match operator {
        Operator::Or => {
            Err("<left || right> is not applicable for integers!")
        }
        Operator::And => {
            Err("<left && right> is not applicable for integers!")
        }
        Operator::Not => {
            Err("<!> is not applicable for two integers!")
        }
        Operator::NotEqual => {
            Ok(Literal::Boolean(left != right))
        }
        Operator::Equal => {
            Ok(Literal::Boolean(left == right))
        }
        Operator::Greater => {
            Ok(Literal::Boolean(left > right))
        }
        Operator::GreaterOrEqual => {
            Ok(Literal::Boolean(left >= right))
        }
        Operator::Less => {
            Ok(Literal::Boolean(left < right))
        }
        Operator::LessOrEqual => {
            Ok(Literal::Boolean(left <= right))
        }
        Operator::Plus => {
            Ok(Literal::Integer(left + right))
        }
        Operator::Minus => {
            Ok(Literal::Integer(left - right))
        }
        Operator::Divide => {
            //TODO do we always want an integer or better transform to decimal???
            Ok(Literal::Integer(left / right))
        }
        Operator::Multiply => {
            Ok(Literal::Integer(left * right))
        }
        Operator::PowerOf => {
            Err("<left^right> is not applicable for decimals!")
        }
    }
}

/// this function evaluates all operators with decimal input
pub fn eval_decimal_literals(operator: Operator, left: f64, right: f64) -> Result<Literal, &'static str> {

    match operator {
        Operator::Or => {
            Err("<left || right> is not applicable for integers!")
        }
        Operator::And => {
            Err("<left && right> is not applicable for integers!")
        }
        Operator::Not => {
            Err("<!> is not applicable for two integers!")
        }
        Operator::NotEqual => {
            Ok(Literal::Boolean(left != right))
        }
        Operator::Equal => {
            Ok(Literal::Boolean(left == right))
        }
        Operator::Greater => {
            Ok(Literal::Boolean(left > right))
        }
        Operator::GreaterOrEqual => {
            Ok(Literal::Boolean(left >= right))
        }
        Operator::Less => {
            Ok(Literal::Boolean(left < right))
        }
        Operator::LessOrEqual => {
            Ok(Literal::Boolean(left <= right))
        }
        Operator::Plus => {
            Ok(Literal::Decimal(left + right))
        }
        Operator::Minus => {
            Ok(Literal::Decimal(left - right))
        }
        Operator::Divide => {
            Ok(Literal::Decimal(left / right))
        }
        Operator::Multiply => {
            Ok(Literal::Decimal(left * right))
        }
        Operator::PowerOf => {
            Err("<left^right> is not applicable for decimals!")
        }
    }
}