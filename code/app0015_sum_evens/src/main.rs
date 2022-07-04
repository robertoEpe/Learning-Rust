// The sum of the first n even numbers is given by n*(n + 1)
// You can also try the sum of odd numbers n*n or the fist n natural numbers  n*(n + 1) / 2

fn main() {
    let mut sum_of_evens = 0;
    for i in 1..=20 {
        sum_of_evens += i * 2;
        println!("{}({}) -> {} ==> n * (n + 1) == {}", i, i*2, sum_of_evens, i *(i + 1));

    }
}
