fn main() {
    println!("Enter a number: ");

    // Read the line using our function and parse to float
    let number: f64 = get_value(); //  unwrap_or let us use a defualt value if it fails
    println!("The {} * 2 + 1 = {}", number, number * 2.0 + 1.0);
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
