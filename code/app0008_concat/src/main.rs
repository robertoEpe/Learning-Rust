mod utils;
use utils::read_line;
fn main() {
    println!("Please enter your first name:");
    let first_name = read_line();
    println!("Please enter your last name:");
    let last_name = read_line();

    // Using format to create a new string
    let name = format!("{} {}", &first_name, &last_name);
    println!("{}", name);

}
