pub fn valid_parenthesis(s: String) -> bool {
    // []
    // {}
    // ()
    //
    let mut input = s;

    // for parens pair
    // while input contains parens pair remove it
    //
    let mut contains_bracket = input.contains("[]");
    let mut contains_fancy = input.contains("{}");
    let mut contains_round = input.contains("()");

    while contains_round || contains_fancy || contains_bracket {
        contains_fancy = input.contains("{}");
        contains_bracket = input.contains("[]");
        contains_round = input.contains("()");
        if contains_bracket {
            input = input.replace("[]", "");
        } else if contains_fancy {
            input = input.replace("{}", "");
        } else {
            input = input.replace("()", "");
        }
    }
    input.len() == 0

    //  make an enum of paren types, and have a stack. When you find an opening paren,
    //  push that to the stack. When you find a closing paren, pop the top value from the stack
    //  , and if it is of the correct paren type, continue. Otherwise, return false.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_parenthesis_test() {
        assert!(valid_parenthesis("{[]}".to_owned()));
    }
}
