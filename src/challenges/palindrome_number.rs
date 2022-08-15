/// Returns `true` if number is palindrome and `false` if not.
///
/// # Arguments 🧩
///
/// * `x` - integer of type `i32` that represents the number
///
/// # Returns ⏎
///
/// * `bool`
///
/// # Examples 📗
/// ```
/// let num1 = 121;
/// assert_eq!(palindrome_number(num1), true);
///
/// let num2 = 223;
/// assert_eq!(palindrome_number(num2), false);
/// ```
pub fn palindrome_number(x: i32) -> bool {
    let number_string = x.to_string();
    let res: String = number_string.chars().rev().collect();
    res == number_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_number_test() {
        assert_eq!(palindrome_number(121), true);
        assert_eq!(palindrome_number(12), false);
        assert_eq!(palindrome_number(222), true);
    }
}
