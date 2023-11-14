use std::env;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Veuillez fournir trois entiers.");
        return;
    }
    let a = parse_arg(&args[1]);
    let b = parse_arg(&args[2]);
    let c = parse_arg(&args[3]);

    match (a, b, c) {
        (Ok(val1), Ok(val2), Ok(val3)) => {
            find_middle_value(val1, val2, val3);
        }
        _ => println!("Veuillez fournir des entiers valides."),
    }
}

fn parse_arg(arg: &str) -> Result<i32, ParseIntError> {
    arg.parse::<i32>()
}

fn find_middle_value(a: i32, b: i32, c: i32) {
    if (a > b) != (a > c) {
        println!("{}", a);
    } else if (b > a) != (b > c) {
        println!("{}", b);
    } else if (a == b) && (b == c) {
        println!("erreur.");
    } else {
        println!("{}", c);
    }
}
