
fn main(){
    println!("-Start");
    println!("ALPHABET:> ");

    let lettre   = 'a' ;
    let u8_lettre = lettre as u8 ;
    for i in u8_lettre..=122  // 122 Magic Number
    {
        print!("{}", i as char) ; 
    }
    println!("");
    println!("-END!");
}




