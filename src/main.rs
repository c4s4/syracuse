use std::process;
use std::vec;

fn main() {
    for arg in std::env::args().skip(1) {
        let n: u64 = match arg.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("{arg} is not a number!");
                process::exit(1);
            }
        };
        syracuse(n);
    }
}

fn syracuse(mut n: u64) {
    let mut max: u64 = n;
    let mut values: Vec<String> = vec![n.to_string()];
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        if n > max {
            max = n;
        }
        values.push(n.to_string());
    }
    println!("{} ({}) [{}]", values.join(" "), values.len(), max);
}
