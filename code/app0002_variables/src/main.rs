fn main() {
    /*
    Variables are where we put our data.
    In Rust variables are immutable by default
    */
    let name = "Roberto";

    // You can interpolate, you primary data using {} and passing the variable name
    println!("Hello, {}!", name);
    
    // You can re-declare the same varible. This is called shadowing. Notice is not the same type
    let name = name.to_uppercase(); // We are using a method.

    println!("Big name, {}!", name);
}
