use std::process;
use std::vec;

fn main() {
    for arg in std::env::args().skip(1) {
        let result = arg.trim().parse();
        if result.is_err() {
            eprintln!("ERROR {} is not a valid number", arg);
            process::exit(1);
        }
        syracuse(result.unwrap());
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
