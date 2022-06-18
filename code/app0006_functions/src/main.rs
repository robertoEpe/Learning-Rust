fn main() {
    println!("Enter a number: ");
    
    // Read the line using our function and parse to float
    let number:f64 = read_line().parse().unwrap_or(0.0); //  unwrap_or let us use a defualt value if it fails
    println!("The double plus one is: {}", number * 2.0 + 1.0);
}

// declare a function that returs a String. This is cheap because it moves by the value.
fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin().read_line(&mut result).expect("Failed to read a line from stdin");
    result = result.trim().to_string();
    result
}
