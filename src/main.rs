use challenges::{reverse_integer::reverse, pow::my_pow};



mod challenges;

fn main() {
    println!("{}", my_pow(2.0, 4));
    println!("{}", f64::powf(2.0, 10.0));
}

fn fizz_buzz() {
    let mut output = String::new();
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            output += "Fizz_Buzz";
        } else if i % 5 == 0 {
            output += "Buzz";
        } else if i % 3 == 0 {
            output += "Fizz";
        } else {
            output += &format!("{}", i);
        }
    }
    println!("{output}");
}
