fn main () {
	let a1:f64 = 450000.0;
	let a2:f64 = 1500000.0;
	let a3:f64 = 750000.0;
	let a4:f64 = 2850000.0;
	let a5:f64 = 250000.0;  

	let sum = a1 + a2 + a3 + a4 + a5;
	println!("Sum is {}", sum);

	let average = sum / 5.0; 
	println!("Average is {}", average);
}