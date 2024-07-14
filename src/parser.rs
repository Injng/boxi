/// represents a Number, which can be either binary, octal, decimal, or hex
#[derive(Debug)]
pub enum Number {
    Binary(i64),
    Octal(i64),
    Decimal(i64),
    Hexadecimal(i64),
}

/// represents an Operator, which operates on two Numbers
#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// represents a Token, which can be either a Number or an Operator
#[derive(Debug)]
pub enum Token {
    Number(Number),
    Operator(Operator),
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

/// validate if a string is a valid expression
pub fn validate(expr: &str) -> bool {
    // separate the string into substrings, with each divided by an Operator
    let mut start = 0;
    let mut end = 0;
    while end < expr.len() {
        if match_operator(&expr, end).is_some() {
            // if end is the same as start, then the Operator is invalid
            if end == start { return false; }
            // otherwise, check if the substring leading up to the Operator is a valid Number
            let substr = &expr[start..end];
            if match_number(substr.trim()).is_none() { return false; }
            start = end + 1;
        }
        end += 1;
    }
    // check if the last substring is a valid Number
    let substr = &expr[start..end];
    if match_number(substr.trim()).is_none() { return false; }
    return true;
}

/// parses a string into a list of Tokens
pub fn parse(expr: &str) -> Result<Vec<Token>, &str> {
    // first check if expression is valid
    if !validate(expr) { return Err("invalid expression"); }

    // separate the string into substrings, with each divided by an Operator
    let mut start = 0;
    let mut end = 0;
    let mut tokens = Vec::new();
    while end < expr.len() {
        let operator = match_operator(&expr, end);
        if operator.is_some() {
            let substr = &expr[start..end];
            let num = match_number(substr.trim()).unwrap();
            tokens.push(Token::Number(num));
            tokens.push(Token::Operator(operator.unwrap()));
            start = end + 1;
        }
        end += 1;
    }

    // push the last Number substring
    let substr = &expr[start..end];
    let num = match_number(substr.trim()).unwrap();
    tokens.push(Token::Number(num));

    return Ok(tokens);
}

