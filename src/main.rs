

mod lexer;
mod parser;
mod interpreter;

enum CalculatorError {
    CouldNotReadLine(std::io::Error),
}

fn main() {
    println!("                Hello, world! This is a simple calculator.");
    println!("                **To quit, press \"q\" on a new line, then hit enter**");
    println!("                Enter any expression on the following line:");

    while let Ok(input) = get_input() {
        if input == String::from("q\n") {
            break;
        }

        match lexer::lex(&input) {
            Ok(tokens) => {
                //println!("DEBUG: The tokens are {:?}", tokens);
                match parser::parse(tokens) {
                    Ok(node) => {
                        //println!("DEBUG: The AST is {:?}", node);
                        match interpreter::interpret(node) {
                            Ok(result) => {
                                println!("                {}", result);
                            },
                            Err(_) => {
                                println!("                ERROR: interpreter. This should not happen.");
                            }
                        }
                    },
                    Err(parser::ParserError::UnexpectedToken) => {
                        println!("                ERROR: Found ununexpected token");
                    },
                    Err(parser::ParserError::UnexpectedEOF) => {
                        println!("                ERROR: Need to write more at the end");
                    },
                    Err(parser::ParserError::UnclosedParenthesis) => {
                        println!("                ERROR: Unclosed parenthesis");
                    }
                }
            },
            Err(lexer::LexerError::UnknownToken(t)) => {
                println!("                ERROR: Found unknown token: {}", t);
            },
            Err(lexer::LexerError::NumberNotParseable(n)) => {
                println!("                ERROR: Number could not be parsed: {}", n);
            }
        }
    }

    println!("                End Calculator");
}


fn get_input() -> Result<String, CalculatorError> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        Err(error) => Err(CalculatorError::CouldNotReadLine(error))
    }
}