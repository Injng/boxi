use crate::parser::{ Number, Operator, Paran, Token };

/*
struct Rosetta {
    num: i64,
}

impl Rosetta {
    fn init(num: i64) -> Rosetta {
        return Rosetta { num };
    }

    fn to_binary(&self) -> String {
        return format!("{:b}", self.num);
    }

    fn to_octal(&self) -> String {
        return format!("{:o}", self.num);
    }

    fn to_hexadecimal(&self) -> String {
        return format!("{:x}", self.num);
    }
}
*/

/// uses a shunting algorithm to convert a vector of tokens into postfix notation
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

/*
fn interpret(tokens: Vec<Token>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    for token in tokens.iter() {
        match token {
            Token::Number(Number::Binary(num)) => stack.push(*num),
            Token::Number(Number::Octal(num)) => stack.push(*num),
            Token::Number(Number::Decimal(num)) => stack.push(*num),
            Token::Number(Number::Hexadecimal(num)) => stack.push(*num),
            Token::Operator(Operator::Add) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            },
            Token::Operator(Operator::Subtract) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a - b);
            },
            Token::Operator(Operator::Multiply) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            },
            Token::Operator(Operator::Divide) => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a / b);
            },
        }
    }
    return stack.pop().unwrap();
}
*/
