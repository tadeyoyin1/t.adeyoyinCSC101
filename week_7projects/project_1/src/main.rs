use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * (side * side)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    3.14159 * radius * radius * height
}

fn main() {
    let mut input = String::new();
    println!("Select calculation:");
    println!("1 = Area of Trapezium");
    println!("2 = Area of Rhombus");
    println!("3 = Area of Parallelogram");
    println!("4 = Area of Cube");
    println!("5 = Volume of Cylinder");

    io::stdin().read_line(&mut input).unwrap();
    let choice: i32 = input.trim().parse().unwrap_or(0);

    match choice {
        1 => {
            let (height, base1, base2) = get_three_inputs();
            println!("Result: {}", area_of_trapezium(height, base1, base2));
        }
        2 => {
            let (diagonal1, diagonal2) = get_two_inputs();
            println!("Result: {}", area_of_rhombus(diagonal1, diagonal2));
        }
        3 => {
            let (base, altitude) = get_two_inputs();
            println!("Result: {}", area_of_parallelogram(base, altitude));
        }
        4 => {
            let side = get_one_input();
            println!("Result: {}", area_of_cube(side));
        }
        5 => {
            let (radius, height) = get_two_inputs();
            println!("Result: {}", volume_of_cylinder(radius, height));
        }
        _ => println!("Invalid selection"),
    }
}

fn get_one_input() -> f64 {
    let mut input = String::new();
    println!("Enter value:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0.0)
}

fn get_two_inputs() -> (f64, f64) {
    let mut input = String::new();
    println!("Enter first value:");
    io::stdin().read_line(&mut input).unwrap();
    let first = input.trim().parse().unwrap_or(0.0);

    input.clear();
    println!("Enter second value:");
    io::stdin().read_line(&mut input).unwrap();
    let second = input.trim().parse().unwrap_or(0.0);

    (first, second)
}

fn get_three_inputs() -> (f64, f64, f64) {
    let mut input = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut input).unwrap();
    let height = input.trim().parse().unwrap_or(0.0);

    input.clear();
    println!("Enter first base:");
    io::stdin().read_line(&mut input).unwrap();
    let base1 = input.trim().parse().unwrap_or(0.0);

    input.clear();
    println!("Enter second base:");
    io::stdin().read_line(&mut input).unwrap();
    let base2 = input.trim().parse().unwrap_or(0.0);

    (height, base1, base2)
}
