fn main() {
    // We add the keyworkd mut after let and before the variable name if we want it to be mutable
    let mut x = 10;
    println!("My number is: {}", x);

    const INCREASE: i32 = 5; // Define a constant. It needs the type

    x = INCREASE + x; // Increase the old value of x, and assigned to x again
    println!("Now my number is: {}", x);
    
}
