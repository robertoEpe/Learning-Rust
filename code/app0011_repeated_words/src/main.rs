mod utils;
use utils::read_line;

fn main() {
    println!("Enter a list of words:");

    let word_list = read_line();
    let mut prev: &str = "";
    for word in word_list.split_whitespace() {
        if prev == word {
            println!("Reapeat word {}", word);
        }
        prev = word;
    }
}
