use std::env;

#[warn(unused)]
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("there is no file passed!");
        return
    }
    let file_lines = std::fs::read_to_string(&args[1]).expect("file not found!");
    for current in file_lines.split("\n") {
        let num: i64 = current.parse().unwrap();
        if is_prime(num) {
            println!("{}", 1);
        }else {
            println!("{}", 0);
        }
    }
}

fn is_prime(number: i64) -> bool {
    if number <= 1 {
        return false;
    }
    if number <=3 {
        return true;
    }
    if number % 2 == 0 || number % 3 == 0{
        return false;
    }
    let num_sqrt = (number as f64).sqrt();
    for i in 2..num_sqrt as i64 {
        if number % i == 0 {
            return true;
        }
    }
    true
}
