use std::io;

fn main() {
    let mut input = String::new();
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaike Soup (N3200)");
    println!("F = Fried Rice & Chicken (N3000)");
    println!("A = Amala & Ewedu Soup (N2500)");
    println!("E = Eba & Egusi Soup (N2000)");
    println!("W = White Rice & Stew (N2500)");
    
    let prices = [("P", 3200), ("F", 3000), ("A", 2500), ("E", 2000), ("W", 2500)];
    
    println!("Enter food type: ");
    io::stdin().read_line(&mut input).unwrap();
    let food_type = input.trim();
    
    input.clear();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut input).unwrap();
    let quantity: i32 = input.trim().parse().unwrap_or(0);
    
    let total = prices.iter()
                      .find(|&&(f, _)| f == food_type)
                      .map(|&(_, price)| price * quantity)
                      .unwrap_or(0);

    let final_total = if total > 10000 {
        (total as f64 * 0.95) as i32
    } else {
        total
    };

    println!("Total charges: N{}", final_total);
}