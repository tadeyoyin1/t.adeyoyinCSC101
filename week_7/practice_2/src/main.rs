use std::io;
fn checker() {
    let mut input = String::new();
    println!("Enter a charcater:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <='9'
    {
        println!("charcater '{}' is a digit",ch);
    }
    else {
        println!("Charcter '{}' is not a digit",ch);
    }
}

fn main(){
    println!("Welcome! this program checks whether a charcater variable contains
       a digit or not ");
    checker()
}