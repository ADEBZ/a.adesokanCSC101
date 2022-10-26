fn  main() {
	let t = 2.0*450_000.00;//toshiba price multiplied by quantity
	let m = 1.0*1_500_000.00;//mac price multiplied by quantity
	let h = 3.0*750_000.00;//hp price multiplied by quantity
	let d = 3.0* 2_850_000.00;//dell price multiplied by quantity
	let a = 1.0*250_000.00;//acer price multiplied by quantity

	let sum = t+m+h+d+a; //sum of all prices
	let avg = sum/(2.0+1.0+3.0+3.0+1.0);

	println!("the sum of the sales record is {}",sum );
	println!("the average of the sales record is {}",avg );
}