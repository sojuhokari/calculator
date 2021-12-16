use crate::lexer::Token;
use std::slice::Iter;
use std::iter::Peekable;


#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    UnexpectedToken,
    UnclosedParenthesis,
}

#[derive(Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Number(f64),
}

pub fn parse(tokens: Vec<Token>) -> Result<Node, ParserError> {
    let mut iter = tokens.iter().peekable();
    parse_expr(&mut iter)
}

pub fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
    parse_add_sub(tokens)
}

pub fn parse_add_sub(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
    let mut expr = parse_mult_div(tokens)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus => {
                tokens.next();
                let rhs = parse_mult_div(tokens)?;
                expr = Node::Add(Box::new(expr), Box::new(rhs));
            },
            Token::Minus => {
                tokens.next();
                let rhs = parse_mult_div(tokens)?;
                expr = Node::Subtract(Box::new(expr), Box::new(rhs));
            },
            _ => break
        }
    }

    Ok(expr)
}

pub fn parse_mult_div(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
    let mut expr = parse_number(tokens)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Times => {
                tokens.next();
                let rhs = parse_number(tokens)?;
                expr = Node::Multiply(Box::new(expr), Box::new(rhs));
            },
            Token::Divide => {
                tokens.next();
                let rhs = parse_number(tokens)?;
                expr = Node::Divide(Box::new(expr), Box::new(rhs));
            },
            _ => break
        }
    }

    Ok(expr)
}

pub fn parse_number(tokens: &mut Peekable<Iter<Token>>) -> Result<Node, ParserError> {
    match tokens.next() {
        Some(Token::Number(n)) => {
            Ok(Node::Number(*n))
        },
        Some(Token::OpenParenthesis) => {
            let expr = parse_add_sub(tokens);
            match tokens.next() {
                Some(Token::CloseParenthesis) => expr,
                Some(_) => Err(ParserError::UnexpectedToken),
                None => Err(ParserError::UnclosedParenthesis),
            }
        },
        Some(_) => Err(ParserError::UnexpectedToken),
        None => Err(ParserError::UnexpectedEOF),
    }
}