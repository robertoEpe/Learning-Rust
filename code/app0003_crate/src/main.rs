fn main() {
    // Using the crate rand to generate a random number. We provide the type un the variable declaration
    let number:f32 = rand::random();
    println!("Random float (0:1): {number}"); // We interpolate using a short hand

    // We provide the variable type for number in the function invocation
    let number = rand::random::<u32>();
    println!("Random positive integer of 32 bits: {number}");
}
