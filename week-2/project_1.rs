fn main() {
	let p:f64 = 520_000_000.00;
	let n:f64 = 5.0;
	let r:f64 = 10.0;
	let a = p * (1.0 +(r / 100.00))*n;
	let ci = a-p;
	println!("The annuity is {} and the compound interest is {}", a,ci );
}