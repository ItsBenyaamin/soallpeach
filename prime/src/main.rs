use std::env;

#[warn(unused)]
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("there is no file passed!");
        return
    }
    std::fs::read_to_string(&args[1]).expect("file not found!")
        .lines()
        .for_each(|x| match x.parse::<i64>() {
            Ok(x) => {
                if is_prime(x) {
                    println!("{}", 1)
                }else {
                    println!("{}", 0)
                }
            },
            _ => println!("there is a non-numeric character!")
        });
}

fn is_prime(number: i64) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}