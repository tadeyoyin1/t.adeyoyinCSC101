fn  main() {
	
	let t:f64 = 450_000.00;
	let m:f64 = 1_500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.00;

	let sum = (2.00 * t) + (1.00 * m) + (3.00 * h) + (3.00 * d) + (1.00 * a);
	println!("sum is {}",sum );

	let a:f64 = sum / 5.00;
	println!("a is {}",a );
}
