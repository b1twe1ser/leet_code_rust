pub fn reverse(x: i32) -> i32 {
    if x >= 0 {
        let intteger_string: String = format!("{x}").chars().rev().collect();

        intteger_string.parse::<i32>().unwrap_or(0)
    } else {
        let intteger_string: String = format!("{}", -x).chars().rev().collect();

        -intteger_string.parse::<i32>().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_test() {
        let number = -123;

        let expected = -321;

        assert_eq!(reverse(number), expected);
    }
}
