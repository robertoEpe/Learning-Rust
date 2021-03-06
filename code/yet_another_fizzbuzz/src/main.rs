fn main() {
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) -> () {
    for i in 1..n {
        fizzbuzz(i);
    }
}

fn fizzbuzz(i: u32) -> () {
    if is_divisible_by(i, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(i, 3) {
        println!("fizz");
    } else if is_divisible_by(i, 5) {
        println!("buzz");
    }else {
        println!("{}", i);
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}
