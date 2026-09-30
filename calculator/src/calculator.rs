fn parse(expr: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    let tokens: Vec<char> = expr.chars().filter(|&c| c != ' ').collect();
    let mut stack: Vec<char> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for token in &tokens {
        match token {
            '0'..='9' => stack.push(*token),
            ')' => {
                while !operators.is_empty() && operators.last().unwrap() != &'(' {
                    let op = operators.pop().unwrap();
                    stack.push(op);
                }
                if operators.pop() != Some('(') {
                    return Err("Mismatched parentheses".into());
                }
            }
            '(' => operators.push(*token),
            '+' | '-' | '/' | '*' => {
                while !operators.is_empty()
                    && precedence(token) <= precedence(operators.last().unwrap())
                {
                    let op = operators.pop().expect("Unexpected end of operators vec");
                    stack.push(op);
                }
                operators.push(*token);
            }
            _ => {
                return Err(format!("Invalid token: {}", token).into());
            }
        }
    }

    while let Some(op) = operators.pop() {
        if op == '(' {
            return Err("Mismatched parentheses".into());
        }
        stack.push(op);
    }

    Ok(stack)
}

fn precedence(op: &char) -> u8 {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

pub fn calculate(exp: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let tokens = parse(exp)?;
    let mut stack: Vec<i32> = Vec::new();
    for token in tokens {
        match token {
            '0'..='9' => match token.to_digit(10) {
                Some(num) => stack.push(num as i32),
                None => {
                    return Err(format!("Unable to parse digit: {}", token).into());
                }
            },
            '+' | '-' | '/' | '*' => {
                let n2 = stack.pop().ok_or("Missing right-hand operand")?;
                let n1 = stack.pop().ok_or("Missing left-hand operand")?;
                let res = perform_operation(n1, n2, token)?;
                stack.push(res);
            }
            _ => {
                return Err(format!("Invalid token found while calculating: {}", token).into());
            }
        }
    }
    if stack.len() > 1 {
        Err("Invalid number of operators and operands".into())
    } else {
        match stack.pop() {
            Some(res) => Ok(res),
            None => Err("Empty stack when expected 1 item".into()),
        }
    }
}

fn perform_operation(n1: i32, n2: i32, op: char) -> Result<i32, Box<dyn std::error::Error>> {
    match op {
        '+' => Ok(n1 + n2),
        '-' => Ok(n1 - n2),
        '*' => Ok(n1 * n2),
        '/' => {
            if let Some(res) = n1.checked_div(n2) {
                Ok(res)
            } else {
                Err(format!("Invalid operator: {}", op).into())
            }
        }
        _ => Err(format!("Invalid operator: {}", op).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication_precidence_over_addition() {
        assert!(precedence(&'*') > precedence(&'+'));
    }

    #[test]
    fn test_division_precidence_over_subtraction() {
        assert!(precedence(&'/') > precedence(&'-'));
    }

    #[test]
    fn test_division_and_multiplication_have_same_precedence() {
        assert_eq!(precedence(&'/'), precedence(&'*'));
    }

    #[test]
    fn test_parse_with_single_operator_expression() {
        let exp = "2 + 3";
        let output = parse(exp).unwrap();
        let expected = vec!['2', '3', '+'];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_with_multiple_precedented_operators_expression() {
        let exp = "2 + 3 * 5";
        let output = parse(exp).unwrap();
        let expected = vec!['2', '3', '5', '*', '+'];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_with_paranthesis_in_expression() {
        let exp = "(2 + 3) * 5";
        let output = parse(exp).unwrap();
        let expected = vec!['2', '3', '+', '5', '*'];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_with_same_precedence_in_expression() {
        let exp = "2 / 3 * 5";
        let output = parse(exp).unwrap();
        let expected = vec!['2', '3', '/', '5', '*'];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_calculate_single_operator_expression_succeeds() {
        let exp = "2 + 3";
        let output = calculate(exp).unwrap();
        assert_eq!(output, 5_i32);
    }

    #[test]
    fn test_calculate_multiple_operator_expression_succeeds() {
        let exp = "2 + 3 * 5";
        let output = calculate(exp).unwrap();
        assert_eq!(output, 17_i32);
    }

    #[test]
    fn test_calculate_divided_by_zero_returns_error() {
        let exp = "2 / 0";
        let output = calculate(exp);
        assert!(output.is_err());
    }

    #[test]
    fn test_calculate_expression_with_paranthesis_succeeds() {
        let exp = "(2 + 3) * 5";
        let output = calculate(exp).unwrap();
        assert_eq!(output, 25_i32);
    }
}
