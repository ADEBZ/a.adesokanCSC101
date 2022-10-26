fn main() {
	let p:f64 = 210_000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;

	 let a = p*(1.00-(r/100.00));
	 let a = f64::powf(a,n);
	 let c:f64 = a-p ;
	 println!("The product depreciated by {} naira",c );

}