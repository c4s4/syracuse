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
        let suite = syracuse(n);
        print(&suite);
    }
}

fn syracuse(mut n: u64) -> Vec<u64> {
    let mut suite: Vec<u64> = vec![n];
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        suite.push(n);
    }
    suite
}

fn print(suite: &Vec<u64>) {
    let max = suite.iter().max().unwrap();
    let strings = suite.iter().map(|n| n.to_string()).collect::<vec::Vec<String>>();
    println!("{} ({}) [{}]", strings.join(" "), suite.len(), max);
}
