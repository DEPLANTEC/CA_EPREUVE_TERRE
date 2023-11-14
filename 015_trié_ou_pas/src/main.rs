use std::env;
use std::num::ParseIntError;
use colored::{*};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}","Veuillez fournir une liste d'entiers.".white().on_red().bold());
        return;
    }
    let mut numbers: Vec<i32> = Vec::new();
    for arg in &args[1..] {
        match arg.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                println!("{}","erreur.".white().on_red().bold());
                return;
            }
        }
    }

    if is_sorted(&numbers) {
        println!("{}","100% Triée, Propre !".white().on_red().bold());
    } else {
        println!("{}","0% Triée, Bordel !! ".white().on_red().bold());
    }
}
fn is_sorted(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] > numbers[i + 1] {

            return false;
        }
    }
    true
}
