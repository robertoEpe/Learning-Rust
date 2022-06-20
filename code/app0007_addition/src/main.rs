use std::io::Write;
mod utils;

fn main() {
    print!("Enter first integer: ");
    std::io::stdout().flush().expect("some error message");

    let number1: i32 = utils::get_value();

    print!("Enter second integer: ");
    std::io::stdout().flush().expect("some error message");

    let number2: i32 = utils::get_value();

    let sum = number1 + number2;

    println!("{} + {} = {}", number1, number2, sum);
}

