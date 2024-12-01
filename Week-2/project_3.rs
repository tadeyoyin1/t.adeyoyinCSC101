fn main() {
	
	let p:f64 = 510_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	let a = p * (1.00 - (r / 100.00)) * (n as f64 ).powf(2.00) ;
	println!("a is {}",a );

}