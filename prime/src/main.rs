use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("there is no file passed!");
        return
    }
    let mut output = String::new();
    let file = File::open(&args[1]).unwrap();
    let lines = BufReader::new(&file).lines().filter_map(std::io::Result::ok);
    for i in lines{
        let num: u64 = i.parse().unwrap();
        output += is_prime(num);
    }
    print!("{}", output);
}

fn is_prime(number: u64) -> &'static str {
    if number <= 1 {
        return "\n0";
    }
    if number <=3 {
        return "\n1";
    }
    if number % 2 == 0 || number % 3 == 0 {
        return "\n0";
    }
    let num_sqrt = (number as f64).sqrt() as u64 ;
    for i in (3..num_sqrt).step_by(2){
        if number % i == 0 {
            return "\n0";
        }
    }
    "\n1"
}