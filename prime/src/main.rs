use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("there is no file passed!");
        return
    }
    let file = File::open(&args[1]).unwrap();
    BufReader::new(file)
        .lines()
        .filter_map(std::io::Result::ok)
        .for_each(|x| {
            let num: i64 = x.parse().unwrap();
            if is_prime(num) {
                println!("{}", 1);
            }else {
                println!("{}", 0);
            }
        });
}

fn is_prime(number: i64) -> bool {
    if number <= 1 || number % 2 == 0 || number % 3 == 0{
        return false;
    }
    if number <=3 {
        return true;
    }
    let num_sqrt = (number as f64).sqrt() as i64;
    for i in 2..num_sqrt {
        if number % i == 0 {
            return true;
        }
    }
    true
}
