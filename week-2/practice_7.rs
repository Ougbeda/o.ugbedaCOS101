fn main () {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//depreciation
	let a = p * (1.0 - (r / 100.0)) .powf(n);
	println!("Value of TV after 3 years is {}", a);

	

}
