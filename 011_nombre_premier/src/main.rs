use std::env;
use std::num::ParseIntError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez taper un entier comme argument.");
        return;
    }
    match parse_arg(&args[1]) {
        Ok(num) if num > 1 => {
            if is_prime(num) {
                println!("Oui, {} est un nombre premier.", num);
            } else {
                println!("Non, {} n'est pas un nombre premier.", num);
            }
        }
        _ => {
            println!("Veuillez taper un entier supérieur à 1.");
        }
    }
    println!("{}", is_prime_wilson(parse_arg(&args[1]).unwrap() )); // Affiche true
}
fn parse_arg(arg: &str) -> Result<i32, ParseIntError> {
    arg.parse::<i32>()
}
fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_prime_wilson(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    (2..n).fold(1, |acc, x| acc * x % n) == n - 1
    // La ligne de code `(2..n).fold(1, |acc, x| acc * x % n) == n - 1`
    // calcule la factorielle modulo de `n - 1` en multipliant
    // progressivement les nombres de 2 à `n - 1`, appliquant le modulo
    // `n` à chaque étape, et vérifie ensuite si le résultat correspond
    // à `n - 1`, ce qui est une condition du théorème de Wilson pour
    // déterminer si `n` est un nombre premier.
}