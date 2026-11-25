use std::collections::VecDeque;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(Operator)
}

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Star,
    Slash
}

#[derive(Debug)]
enum ParserError {
    WrongNumberOfDots(usize),
    WrongCharacter(char, usize),
    InvalidNumber(String, usize)
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongNumberOfDots(p) => write!(f, "A seconds dot on a number was found at position {p}"),
            Self::WrongCharacter(c, p) => write!(f, "The character {c} is not recognized at position {p}"),
            Self::InvalidNumber(text, position) => write!(f, "Invalid number '{text}' at position {position}")
        }
    }
}

impl Error for ParserError {}

fn parse(s: &str) -> Result<Vec<Token>, ParserError> {
    let mut chars = s.chars().enumerate().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(&(i, c)) = chars.peek() {

        if c.is_ascii_digit() {
            let mut buffer = String::new();
            let mut already_a_dot = false;

            while let Some(&(i, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    buffer.push(c);
                    chars.next();
                } else if c == '.' {
                    if already_a_dot {
                        return Err(ParserError::WrongNumberOfDots(i));
                    }
                    buffer.push(c);
                    already_a_dot = true;
                    chars.next();
                } else {
                    break;
                }
            }

            tokens.push(Token::Number(buffer.parse().map_err(|_| ParserError::InvalidNumber(buffer, i))?));
            continue;
        }

        match c {
            '+' => tokens.push(Token::Operator(Operator::Plus)),
            '-' => tokens.push(Token::Operator(Operator::Minus)),
            '*' => tokens.push(Token::Operator(Operator::Star)),
            '/' => tokens.push(Token::Operator(Operator::Slash)),
            _ if c.is_whitespace() => (),
            c => return Err(ParserError::WrongCharacter(c, i))
        }

        chars.next();
    }

    Ok(tokens)
}

fn shutting_yard(tokens: Vec<Token>) {
    let mut queue = VecDeque::<Token>::new();

    for token in tokens {
        match token {
            Token::Number(n) => {
                queue.push_back(token);
            },
            Token::Operator(op) => {
                
            }
            _ => ()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let tokens = parse(&line)?;

    println!("{:?}", tokens);

    Ok(())
}
