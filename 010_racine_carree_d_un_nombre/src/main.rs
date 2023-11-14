use std::env;
use std::num::ParseIntError;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez taper un entier positif comme argument.");
        return;
    }
    match parse_arg(&args[1]) {
        Ok(num) if num >= 0 => {
            let sqrt = (num as f64).sqrt();
            println!("{}", sqrt as f64);
        }
        _ => {
println!("
                        Veuillez taper un entier
                    ░█▀█░█▀█░█▀▀░▀█▀░▀█▀░▀█▀░█▀▀
                    ░█▀▀░█░█░▀▀█░░█░░░█░░░█░░█▀▀
                    ░▀░░░▀▀▀░▀▀▀░▀▀▀░░▀░░▀▀▀░▀░░
                                          valide.
            ");
        }
    }
}
fn parse_arg(arg: &str) -> Result<i32, ParseIntError> {
    arg.parse::<i32>()
}
