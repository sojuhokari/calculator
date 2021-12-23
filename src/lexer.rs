
#[derive(Debug)]
pub enum LexerError {
    UnknownToken(char),
    NumberNotParseable(String),
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
}

pub fn lex(string: &str) -> Result<Vec<Token>, LexerError> {
    let mut iter = string.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(character) = iter.next() {
        tokens.push(match character {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' | '.' => {
                let mut number_string = String::new();
                number_string.push(character);
                loop {
                    match iter.peek() {
                        Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') | Some('7') | Some('8') | Some('9') | Some('0') | Some('.') => {
                            number_string.push(iter.next().unwrap()) // This unwrap() is ok because we already checked with peek() to make sure it exists
                        },
                        Some(' ') | Some(',') => {
                            iter.next();
                        },
                        _ => {
                            break;
                        }
                    }
                }
                //println!("DEBUG: float before parse is {}", number_string);
                let float: f64 = match number_string.parse() {
                    Ok(n) => n,
                    Err(_) => return Err(LexerError::NumberNotParseable(number_string))
                };
                Token::Number(float)
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Divide,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            ' ' | '\t' | '\n' => {
                continue;
            },
            '\r' => {
                if let Some('\n') = iter.peek() {
                    iter.next();
                }
                continue;
            }
            _ => {
                return Err(LexerError::UnknownToken(character))
            }
        })
    }
    Ok(tokens)
}