#[allow(unused)]
pub fn get_value<T: std::str::FromStr>() -> T {
    match read_line().parse() {
        Ok(n) => n,
        Err(_) => {
            panic!("Failed to parse the value");
        }
    }
}

// declare a function that returs a String. This is cheap because it moves by the value.
pub fn read_line() -> String {
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("Failed to read a line from stdin");
    result = result.trim().to_string();
    result
}
