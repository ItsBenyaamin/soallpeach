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
    let reader = BufReader::new(&file);
    for i in reader.lines(){
            let num: u64 = i.unwrap().parse().unwrap();
            output += format!("{}\n", is_prime(num)).as_str();
    }
    print!("{}", output);
}

fn is_prime(number: u64) -> u8 {
    if number <= 1 {
        return 0;
    }
    if number <=3 {
        return 1;
    }
    if number % 2 == 0 || number % 3 == 0{
        return 0;
    }
    let num_sqrt = (number as f64).sqrt() as u64 ;
    for i in (2..num_sqrt).step_by(2){
        if number % i == 0 {
            return 0;
        }
    }
    1
}
