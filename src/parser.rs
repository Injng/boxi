/// represents a Number, which can be either binary, octal, decimal, or hex
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Binary(i64),
    Octal(i64),
    Decimal(i64),
    Hexadecimal(i64),
}

/// represents an Operator, which operates on two Numbers
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    /// returns the precedence of an Operator
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Add => 1,
            Operator::Subtract => 1,
            Operator::Multiply => 2,
            Operator::Divide => 2,
        }
    }

    /// returns the associativity of an Operator, 0 for left, 1 for right
    pub fn associativity(&self) -> i32 {
        match self {
            Operator::Add => 0,
            Operator::Subtract => 0,
            Operator::Multiply => 0,
            Operator::Divide => 0,
        }
    }
}

/// represents a Paran, which can be either open or closed
#[derive(Debug, Clone, PartialEq)]
pub enum Paran {
    Open,
    Close,
}

/// represents a Token, which can be either a Number, Operator, Paran, or Function
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Paran(Paran),
    // Function,
}

/// given a string, match it to an Operator or return None otherwise
fn match_operator(s: &str, idx: usize) -> Option<Operator> {
    match &s[idx..idx+1] {
        "+" => Some(Operator::Add),
        "-" => Some(Operator::Subtract),
        "*" => Some(Operator::Multiply),
        "/" => Some(Operator::Divide),
        _ => None,
    }
}

/// given a string, match it to a Number or return None otherwise
fn match_number(s: &str) -> Option<Number> {
    if s.starts_with("0b") {
        let num = i64::from_str_radix(&s[2..], 2);
        if num.is_ok() { return Some(Number::Binary(num.unwrap())); }
        else { return None; }
    } else if s.starts_with("0o") {
        let num = i64::from_str_radix(&s[2..], 8);
        if num.is_ok() { return Some(Number::Octal(num.unwrap())); }
        else { return None; }
    } else if s.starts_with("0x") {
        let num = i64::from_str_radix(&s[2..], 16);
        if num.is_ok() { return Some(Number::Hexadecimal(num.unwrap())); }
        else { return None; }
    } else {
        let num = i64::from_str_radix(s, 10);
        if num.is_ok() { return Some(Number::Decimal(num.unwrap())); }
        else { return None; }
    }
}

/// given a string, check if all parantheses are matching
fn check_parantheses(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        if c == '(' { stack.push(c); }
        else if c == ')' {
            if stack.is_empty() { return false; }
            stack.pop();
        }
    }
    return stack.is_empty();
}

/// recursively divide an expression into sub-expressions, and create a vector of Tokens
pub fn tree(expr: &str, tokens: Option<Vec<Token>>) -> Result<Vec<Token>, &str> {
    let mut tokens = tokens.unwrap();

    // match an operator to divide expression into sub-expressions
    let mut start = 0;
    let mut end = 0;
    let mut matched = false;
    while end < expr.len() {
        let operator = match_operator(&expr, end);
        if operator.is_some() {
            if start == end { return Err("consecutive operators"); }
            matched = true;
            let substr = &expr[start..end];
            tokens = tree(substr, Some(tokens.clone()))?;
            tokens.push(Token::Operator(operator.unwrap()));
            start = end + 1;
        }
        end += 1;
    }

    if matched { 
        // do one more check for the final substring
        let substr = &expr[start..end];
        tokens = tree(substr, Some(tokens.clone()))?;
        return Ok(tokens); 
    }
    
    // if no operator was matched, start matching for other elements
    // first check for a number; if a number is matched, then no need to check more
    let num = match_number(&expr[start..end]);
    if num.is_some() { 
        tokens.push(Token::Number(num.unwrap())); 
        return Ok(tokens);
    }
    // otherwise, continue matching for parantheses
    start = 0;
    end = 0;
    let mut last_close = false;
    while end < expr.len() {
        if &expr[end..end+1] == "(" {
            // if a number immediately precedes a parantheses, error
            let num = match_number(&expr[start..end]);
            if num.is_some() {
                return Err("no implied multiplication 2");
            } else if start != end {
                return Err("invalid expression");
            }
            tokens.push(Token::Paran(Paran::Open));
            start = end + 1;
            last_close = false;
        } else if &expr[end..end+1] == ")" {
            let num = match_number(&expr[start..end]);
            if num.is_some() { tokens.push(Token::Number(num.unwrap())); }
            else if start != end {
                return Err("invalid expression");
            }
            tokens.push(Token::Paran(Paran::Close));
            start = end + 1;
            last_close = true;
        }
        end += 1;
    }

    // check if the last substring is a valid Number
    let num = match_number(&expr[start..end]);
    if num.is_some() && last_close {
        return Err("no implied multiplication");
    } else if num.is_some() {
        tokens.push(Token::Number(num.unwrap()));
    } else if start != end {
        return Err("invalid expression");
    }

    return Ok(tokens);
}

/// parses a string into a list of Tokens
pub fn parse(expr: &str) -> Result<Vec<Token>, &str> {
    // first check for matching parantheses
    if !check_parantheses(expr) { return Err("mismatched parantheses"); }
    // then, parse the expression into a list of Tokens
    let tokens: Vec<Token> = tree(expr.trim(), Some(Vec::new()))?;
    Ok(tokens)
}

