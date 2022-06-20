use std::io::Write;

fn main() {
    print!("Enter first integer: ");
    std::io::stdout().flush().expect("some error message");

    let number1: i32 = get_value();

    print!("Enter second integer: ");
    std::io::stdout().flush().expect("some error message");

    let number2: i32 = get_value();

    let sum = number1 + number2;

    println!("{} + {} = {}", number1, number2, sum);
}

fn get_value<T: std::str::FromStr>() -> T {
    match read_line().parse() {
        Ok(n) => n,
        Err(_) => {
            panic!("Failed to parse the value");
        }
    }
}

// declare a function that returs a String. This is cheap because it moves by the value.
fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("Failed to read a line from stdin");
    result = result.trim().to_string();
    result
}
