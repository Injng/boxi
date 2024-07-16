use crate::parser::{ Number, Operator, Paran, Token };

/// uses a shunting yard algorithm to convert a vector of tokens into postfix notation
pub fn shunt(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    for token in tokens.iter() {
        match token {
            Token::Number(_) => output.push(token.clone()),
            Token::Operator(op1) => {
                let mut pushed = false;
                while !stack.is_empty() {
                    let top = stack.last().unwrap().clone();
                    if let Token::Operator(op2) = top {
                        if op1.precedence() > op2.precedence() {
                            pushed = true;
                            stack.push(Token::Operator(op1.clone()));
                            break;
                        } else if op1.precedence() < op2.precedence() {
                            stack.pop();
                            output.push(top.clone());
                        } else if op1.associativity() == 0 { 
                            pushed = true;
                            stack.pop();
                            output.push(top.clone());
                            stack.push(Token::Operator(op1.clone()));
                            break;
                        } else { 
                            pushed = true;
                            stack.push(Token::Operator(op1.clone())); 
                            break;
                        }
                    } else {
                        pushed = true;
                        stack.push(token.clone());
                        break;
                    }
                }
                if !pushed { stack.push(token.clone()); }
            },
            Token::Paran(p) => {
                if *p == Paran::Open {
                    stack.push(Token::Paran(Paran::Open));
                } else {
                    let mut top = stack.last().unwrap().clone();
                    while top != Token::Paran(Paran::Open) {
                        stack.pop();
                        output.push(top.clone());
                        top = stack.last().unwrap().clone();
                    }
                    stack.pop();
                }
            },
        }
    }

    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }

    return output;
}

/// interpret a vector of tokens in postfix notation
pub fn interpret(tokens: Vec<Token>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    for token in tokens.iter() {
        match token {
            Token::Number(num) => {
                match num {
                    Number::Binary(n) => stack.push(*n),
                    Number::Octal(n) => stack.push(*n),
                    Number::Decimal(n) => stack.push(*n),
                    Number::Hexadecimal(n) => stack.push(*n),
                }
            },
            Token::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match op {
                    Operator::Add => stack.push(a + b),
                    Operator::Subtract => stack.push(a - b),
                    Operator::Multiply => stack.push(a * b),
                    Operator::Divide => stack.push(a / b),
                }
            },
            _ => (),
        }
    }

    return stack.pop().unwrap();
}

