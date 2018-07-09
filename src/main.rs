fn main() {
    for n in 0..100 
	{
		let d3 = n%3 == 0;
		let d5 = n%5 == 0;
		
		if d3 && d5 { println!("fizzbuzz") }
		else if d3 { println!("fizz") }
		else if d5 { println!("buzz") }		
		else { println!("{}", n); };	
		
	}	
}
