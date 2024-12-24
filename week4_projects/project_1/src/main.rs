use std::io;

fn main() {
    
    let (a, b, c) = input_coefficients();

    
    let discriminant = b * b - 4 * a * c;

    
    if discriminant > 0 {
        
        let root1 = (-b as f64 + (discriminant as f64).sqrt()) / (2.0 * a as f64);
        let root2 = (-b as f64 - (discriminant as f64).sqrt()) / (2.0 * a as f64);
        println!("Two distinct roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0 {
        
        let root = -b as f64 / (2.0 * a as f64);
        println!("One real root: {:.2}", root);
    } else {
        
        println!("No real roots.");
    }
}

fn input_coefficients() -> (i32, i32, i32) {
    let mut input = String::new();

    println!("Enter the coefficient a:");
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter the coefficient b:");
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();
    input.clear();

    println!("Enter the coefficient c:");
    io::stdin().read_line(&mut input).unwrap();
    let c: i32 = input.trim().parse().unwrap();

    (a, b, c)
}