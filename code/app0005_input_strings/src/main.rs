fn main() {
    println!("Please enter your name:");

    let mut input_string = String::new(); // Create an empty String

    // Using a match to handle the input
    // match std::io::stdin().read_line(&mut input_string) {
    //     Err(n) => eprintln!("{}",n),
    //     Ok(_)=> {
    //         println!("Nice to meet you, {}", input_string);
    //     }
    // }

    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Input string, failed");

    input_string = input_string.trim().to_string(); // Take the new line in the end
    println!("{:?}", input_string); // Debug print
    println!("Nice to meet you, {}", input_string);
}
