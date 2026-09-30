pub fn parse(expr: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    let tokens: Vec<char> = expr.chars().filter(|&c| c != ' ').collect();
    let mut stack: Vec<char> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for token in &tokens {
        match token {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => stack.push(*token),
            ')' => {
                while !operators.is_empty() && operators.last().unwrap() != &'(' {
                    let op = operators.pop().unwrap();
                    stack.push(op);
                }
                operators.pop().expect("Unable to pop ')' from operators");
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
}
