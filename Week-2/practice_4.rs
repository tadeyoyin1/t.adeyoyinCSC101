fn  main() {
	
	let p:f64 = 1000.0;
	let r:f64 = 1.00;
	let t:f64 = 2.00;


	let a = p * (1.0 + (r/100.0)) * t;
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple intrest is {}", si);

	}