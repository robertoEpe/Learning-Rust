use std::io::Write;

fn main() {
    let a = [1,2,3,4,5];

    print!("Please enter an array index: ");
    std::io::stdout().flush().expect("Failed to flush");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse()
    .expect("Index entered was not a number");

    let element = a.get(index).unwrap_or(&-1);

    println!("The value of the element at index {index} is: {element}");
}
