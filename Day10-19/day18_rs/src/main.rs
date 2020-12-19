#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    Digit(u64),
    OpPlus,
    OpTimes,
    LParen,
    RParen,
}

impl Token {
    fn unwrap_digit(self) -> u64 {
        match self {
            Token::Digit(val) => val,
            _ => panic!("Not a digit"),
        }
    }
}

fn parse_expr(expr: &str) -> Vec<Token> {
    expr.chars()
        .filter_map(|c| match c {
            '0'..='9' => Some(Token::Digit(c.to_digit(10).unwrap().into())),
            ')' => Some(Token::RParen),
            '(' => Some(Token::LParen),
            '+' => Some(Token::OpPlus),
            '*' => Some(Token::OpTimes),
            _ => None,
        })
        .collect()
}

fn eval(expr: &Vec<Token>, precedence: bool) -> u64 {
    let mut out: Vec<Token> = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();
    for tok in expr {
        match tok {
            Token::RParen => {
                while op_stack.last() != Some(&Token::LParen) {
                    out.push(op_stack.pop().unwrap());
                }
                if op_stack.last() == Some(&Token::LParen) {
                    op_stack.pop();
                }
            }
            Token::LParen => op_stack.push(*tok),
            Token::OpPlus => {
                while !precedence && op_stack.last() == Some(&Token::OpTimes) {
                    out.push(op_stack.pop().unwrap());
                }
                op_stack.push(*tok)
            }
            Token::OpTimes => {
                while op_stack.last() == Some(&Token::OpPlus) {
                    out.push(op_stack.pop().unwrap());
                }
                op_stack.push(*tok);
            }
            Token::Digit(_) => out.push(*tok),
        }
    }
    while !op_stack.is_empty() {
        out.push(op_stack.pop().unwrap());
    }

    let mut stack: Vec<Token> = Vec::new();
    for tok in out {
        match tok {
            Token::Digit(_) => stack.push(tok),
            Token::OpPlus => {
                let a = stack.pop().unwrap().unwrap_digit();
                let b = stack.pop().unwrap().unwrap_digit();
                stack.push(Token::Digit(a + b))
            }
            Token::OpTimes => {
                let a = stack.pop().unwrap().unwrap_digit();
                let b = stack.pop().unwrap().unwrap_digit();
                stack.push(Token::Digit(a * b))
            }
            _ => {}
        }
    }
    stack.pop().unwrap().unwrap_digit()
}

fn main() {
    let exprs = include_str!("18.txt")
        .lines()
        .map(|l| parse_expr(l))
        .collect::<Vec<_>>();
    println!(
        "Part 1: {}",
        exprs.iter().map(|l| eval(&l, false)).sum::<u64>()
    );
    println!(
        "Part 2: {}",
        exprs.iter().map(|l| eval(&l, true)).sum::<u64>()
    );
}
