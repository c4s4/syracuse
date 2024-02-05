use std::vec;

fn main() {
    for arg in std::env::args().skip(1) {
        let i: u64 = arg.trim().parse().expect("Not a number!");
        syracuse(i);
    }
}

fn syracuse(mut n: u64) {
    let mut max: u64 = n;
    let mut values: Vec<u64> = vec![n];
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        if n > max {
            max = n;
        }
        values.push(n);
    }
    println!("{:?}, max: {max}, count: {}", values, values.len());
}
