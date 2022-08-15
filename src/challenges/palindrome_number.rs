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

/// # Warning 
/// Does not yet work, something isn't working and i honestly doent knwo what :true(
pub fn palindrome_number_no_str(x: i32) -> bool {
    // find out if len is even if even its easy
    let length_number = length_number(x);
    let is_even_digits = length_number % 2 == 0;
    let digits_number = number_to_array(x);

    if is_even_digits {
        let early_indicies = 0..length_number;
        let late_indicies = length_number -1..=0;

        for (i, j) in early_indicies.zip(late_indicies) {
            if digits_number.get(i as usize).unwrap() != digits_number.get(j as usize).unwrap() {
                return false;
            }
        }
    }
    true
}

pub fn number_to_array(number: i32) -> Vec<i32> {
    let mut digits = Vec::<i32>::new();
    let mut count = 0;
    let mut n = number;
    let len_of_number = length_number(number);

    if len_of_number != 0 {
        while n != 0 {
            digits.insert(count, n % 10);
            n /= 10;
            count += 1;
        }
    }
    digits
}

pub fn length_number(number: i32) -> i32 {
    let mut count = 0;
    let mut n = number;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_number_test() {
        assert_eq!(palindrome_number(121), true);
        assert_eq!(palindrome_number(12), false);
        assert_eq!(palindrome_number(222), true);
        assert_eq!(palindrome_number_no_str(122), false);
        assert_eq!(palindrome_number_no_str(121), true);
        assert_eq!(palindrome_number_no_str(223),false);
    }
}
