// Implement negative numbers
pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    } else if x == 0.0 {
        return 0.0;
    }

    let mut result = x;
    let mut counter = 1;
    if n > 0 {
        while counter < n {
            result *= x;
            counter += 1;
        }
    } else {
        while counter < -n {
            result *= -x;
            counter += 1;
        }
        result = 1.0 / result;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn my_pow_test() {
        let num = 2.0;
        let exponent = 10;
        let expected = 1024.00000;

        assert_eq!(my_pow(num, exponent), expected);
        assert_eq!(my_pow(num, -2), 1.0 / 8.0);
    }
}
