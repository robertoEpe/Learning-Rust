// Application to find the largest of an array

fn main() {
    // Creates an array of i32
    let numbers = [17, 22, 8, 35, 5, 1];

    // retrive the first element of the array
    let mut largest = numbers.first().unwrap();

    // Iterates over the reference of the numbers starting with the second element to the end
    for number in &numbers[1..] {

        // if the current numbers is largest than the last largest put the value into the largest variable 
        if number > largest {
            largest = number;
        }
    }

    println!("The array of numbers {:?}", numbers);
    println!("The largets is {largest}");
}
