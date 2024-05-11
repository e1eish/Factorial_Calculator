fn main() {
    let n = 5;
    println!("{}", factorial(n));
}

fn factorial(n: i128) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}