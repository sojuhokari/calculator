use crate::parser::Node;

#[derive(Debug)]
pub enum InterpreterError {
    // TODO
}

pub fn interpret(node: crate::parser::Node) -> Result<String, InterpreterError> {
    let result = interpret_recursively(node)?;
    Ok(format!("{}", result))
}

pub fn interpret_recursively(node: crate::parser::Node) -> Result<f64, InterpreterError> {
    match node {
        Node::Number(n) => Ok(n),
        Node::Add(lhs, rhs) => Ok(interpret_recursively(*lhs)? + interpret_recursively(*rhs)?),
        Node::Subtract(lhs, rhs) => Ok(interpret_recursively(*lhs)? - interpret_recursively(*rhs)?),
        Node::Multiply(lhs, rhs) => Ok(interpret_recursively(*lhs)? * interpret_recursively(*rhs)?),
        Node::Divide(lhs, rhs) => Ok(interpret_recursively(*lhs)? / interpret_recursively(*rhs)?),
    }
}