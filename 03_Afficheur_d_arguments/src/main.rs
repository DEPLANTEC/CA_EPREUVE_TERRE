
use std::env;

fn main() {
    println!("");
    println!("### PROGRAME STARTING #>---------------------------------------------> ");
    let _args:Vec<String> = env::args().collect();
    for i in 1.._args.len(){
        println!("-{}: {}",i,&_args[i])
    }
    println!("<-----------------------------------------------------< TERMINATED ###");
    println!("");
}