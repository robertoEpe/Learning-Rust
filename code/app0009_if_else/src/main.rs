mod utils;
use utils::read_line;
fn main() {
    println!("Please enter a word:");
    let first = read_line();
    println!("Please enter a second word:");
    let second = read_line();

    if first == second {
        println!("That is the same name twice")
    } else if first < second {
        println!("{} is alphabetically before {}", first, second);
    } else {
        println!("{} is alphabetically after {}", first, second);

    }
}
