use std::io;
mod tokenizer;
mod functions;
mod eval;

fn main() {
	println!("Enter your expression:");
	loop{
  	let mut expr: String = String::new();
  	//let expr: String = "int(0, 2pi, sin(x))".to_string();
  	let _ = io::stdin().read_line(&mut expr);
  	let listfromstring: Vec<Vec<String>> = tokenizer::listovac(expr.clone());
  	let result: f64 = eval::evalu8(listfromstring.clone(), 0, listfromstring.len()-1);
  	if functions::floor(result) == result {println!("= {}", result)}
  	else {println!("≈ {}", result)}
  }
}
