// Program to find the size of a variable in bytes

fn main() {
    let val: usize = 23;
    println!("Finding the size of a usize variable : {}", std::mem::size_of_val(&val)); // pass a ref

    let arr: [u16; 3 ] = [10, 11, 12];
    println!("Finding the size of a array [u16;3] variable : {}", std::mem::size_of_val(&arr)); // pass a ref

}
