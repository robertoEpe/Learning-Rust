mod utils;
use utils::get_value;
use std::io::Write;

fn main() {
    println!("Enter two numbers to compare");

    print!("Enter first number: ");
    std::io::stdout().flush().expect("Failed to flush");
    let number1: f64 = get_value();

    print!("Enter second number: ");
    std::io::stdout().flush().expect("Failed to flush");
    let number2: f64 = get_value();

    if number1 == number2 {
        println!("{} == {}", number1, number2);
    }

    if number1 < number2 {
        println!("{} < {}", number1, number2);
    }

    if number1 > number2 {
        println!("{} > {}", number1, number2);
    }

    if number1 <= number2 {
        println!("{} <= {}", number1, number2);
    }

    if number1 >= number2 {
        println!("{} >= {}", number1, number2);
    }

}
