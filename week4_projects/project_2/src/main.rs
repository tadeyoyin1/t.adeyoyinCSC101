use std::io;

fn main() {
    let (experience, age) = input_details();

    let incentive = if experience {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0
        }
    } else {
        100_000
    };

    println!("Annual Incentive: N{}", incentive);
}

fn input_details() -> (bool, u32) {
    let mut input = String::new();

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    let experience = input.trim().eq_ignore_ascii_case("yes");
    input.clear();

    println!("Enter the age of the employee:");
    io::stdin().read_line(&mut input).unwrap();
    let age: u32 = input.trim().parse().unwrap();

    (experience, age)
}