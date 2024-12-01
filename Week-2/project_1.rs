fn main() {
	
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.00;
	let n:f64 = 5.00;

	let a = p * (1.00 + (r / 100.00) ) * n;
	println!("Amount is {}", a );
let cl = a - p; 
println!("Compound intrest is {}",cl );
}