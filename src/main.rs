use std::io;

fn listovac(expr_temp: String) -> Vec<Vec<String>> {
  let mut expr = expr_temp;
  expr = expr.replace(" ", "");
  expr = expr.replace("\n", "");
  //separating:
  /*projíždím charactery expr zleva, přiřadím každému charu typ (a zkontroluji, zda je podporován), nakonec sloučím typy num a rightside_fn/const, které jsou vedle sebe
  	a na závěr rozdělím rightside_fn/const na rightside_fn a const a zkontroluji, jestli jsou sloučené funkce/konstanty podporovány)*/
  let list_of_expr_chars: Vec<char> = expr.chars().collect();
  let mut list: Vec<Vec<String>> = vec![vec![]];
  list.remove(0);
  //0->num 1->() 2->2side_fn 3->chars 4->comma
  let types_of_chars: Vec<Vec<String>> = vec![vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), ".".to_string()], vec!["(".to_string(), ")".to_string()], vec!["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string(), "^".to_string()], vec!["!".to_string(),	"a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string(), "f".to_string(), "g".to_string(), "h".to_string(), "i".to_string(), "j".to_string(), "k".to_string(), "l".to_string(), "m".to_string(), "n".to_string(), "o".to_string(), "p".to_string(), "q".to_string(), "r".to_string(), "s".to_string(), "t".to_string(), "u".to_string(), "v".to_string(), "w".to_string(), "x".to_string(), "y".to_string(), "z".to_string(), "W".to_string()], vec![",".to_string()]];
  let mut last_type: usize = 5;
  for ch in 0..expr.len() {
  	'outer: for k in 0..types_of_chars.len() {
  		for l in 0..types_of_chars[k].len() {
  			if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 0 {
  				if last_type == 0 {
  					let lastindex: usize = list.len()-1;
  					let lastlastindex: usize = list[lastindex].len()-1;
  					list[lastindex][lastlastindex].push(list_of_expr_chars[ch]);
  					break 'outer
  				}
  				else {
  					list.push(vec!["number".to_string(), list_of_expr_chars[ch].to_string()]);
  					last_type = 0;
  					break 'outer
  				}
  			}
  			if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 1 {list.push(vec!["par".to_string(), list_of_expr_chars[ch].to_string()]); last_type = 1; break 'outer}
  			if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 2 {list.push(vec!["binary_fn".to_string(), list_of_expr_chars[ch].to_string()]); last_type = 2; break 'outer}
  			if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 3 {
  			  if last_type == 3 {
  			  	let lastindex: usize = list.len()-1;
  			  	let lastlastindex: usize = list[lastindex].len()-1;
  			  	list[lastindex][lastlastindex].push(list_of_expr_chars[ch]);
  			  	break 'outer
  			  }
  			  else {
  			  	list.push(vec!["unary_fn/const".to_string(), list_of_expr_chars[ch].to_string()]);
  			  	last_type = 3;
  			  	break 'outer
  			  }
  			}
  			if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 4 {list.push(vec!["comma".to_string(), list_of_expr_chars[ch].to_string()]); last_type = 4; break 'outer}
  			if list_of_expr_chars[ch].to_string() != types_of_chars[k][l] && k == 4 && l == types_of_chars[4].len()-1 {println!("ERROR: unsupported character: {}", list_of_expr_chars[ch]); std::process::exit(0)}
  		}
  	}
  }
  let fn_list: Vec<String> = vec![",".to_string(), "ceil".to_string(), "floor".to_string(), "mod".to_string(), "!".to_string(), "sqrt".to_string(), "ln".to_string(), "exp".to_string(), "log".to_string(), "sin".to_string(), "cos".to_string(), "tan".to_string(), "asin".to_string(), "acos".to_string(), "atan".to_string(), "W".to_string()];
  let const_list: Vec<String> = vec!["pi".to_string(), "e".to_string()];
  for ch in 0..list.len() {
  	 	if list[ch][0] == "unary_fn/const" {
  		let mut fn_or_const: bool = false;
  		for k in 0..fn_list.len() {
  			if list[ch][1] == fn_list[k] {
  				list[ch][0] = "unary_fn".to_string(); fn_or_const = true; break
  			}
  		}
  		for k in 0..const_list.len() {
  		  if list[ch][1] == const_list[k] {
  		  	list[ch][0] = "const".to_string(); fn_or_const = true; break
  		  }
  		}
  		if fn_or_const == false {println!("ERROR: unsupported function/constant: {}", list[ch][1]); std::process::exit(0)}
  	}
  }
  //println!("With constants: {:?}", list);
  for ch in 0..list.len() {
		if list[ch][1] == "pi" {list[ch][1] = "3.14159265358979323846".to_string(); list[ch][0] = "number".to_string()}
		if list[ch][1] == "e" {list[ch][1] = "2.71828182845904523536".to_string(); list[ch][0] = "number".to_string()}
		if list[ch][1] == "-" && ch == 0 {
			let mut minuslist: Vec<Vec<String>> = vec![vec!["number".to_string(), "0".to_string()]]; //unary minus => binary minus
			for h in 0..list.len() {minuslist.push(list[h].clone())}
			list = minuslist
		}
		if list[ch][1] == "-" && list[ch-1][1] == "(" {
			let mut minuslist: Vec<Vec<String>> = vec![vec![]];
			minuslist.remove(0);
			for h in 0..ch {minuslist.push(list[h].clone())}
			minuslist.push(vec!["number".to_string(), "0".to_string()]);
			for h in ch..list.len() {minuslist.push(list[h].clone())}
			list = minuslist
		}
		if list[ch][1] == "-" && list[ch-1][1] == "," {
			let mut minuslist: Vec<Vec<String>> = vec![vec![]];
			minuslist.remove(0);
			for h in 0..ch {minuslist.push(list[h].clone())}
			minuslist.push(vec!["number".to_string(), "0".to_string()]);
			for h in ch..list.len() {minuslist.push(list[h].clone())}
			list = minuslist
		}
	}
  list
}

fn plus(a: f64, b: f64) -> f64 {a + b}
fn minus(a: f64, b: f64) -> f64 {a - b}
fn times(a: f64, b: f64) -> f64 {a * b}
fn divide(a: f64, b: f64) -> f64 {
	if b == 0.0 {println!("ERROR: division by zero"); std::process::exit(0)}
	a / b
	}
fn power(a: f64, b: f64) -> f64 {a.powf(b)}
fn fact(n: f64) -> f64 {
	for k in 0..=200 {
		if n == k as f64 {break}
		if k == 200 {println!("ERROR: factorial: argument is not one of 0..=200"); std::process::exit(0)}
	}
	if n == 0.0 {return 1.0}
	n*fact(n-1.0)
}
fn sin(a: f64) -> f64 {
	let mut x: f64 = a;
	if x > 6.2831853071795864769 {
		loop {
			if x < 6.2831853071795864769 {break}
			x -= 6.2831853071795864769;
		}
	}
	if x < -6.2831853071795864769 {
		loop {
			if x > -6.2831853071795864769 {break}
			x += 6.2831853071795864769;
		}
	}
	let mut res: f64 = 0.0;
	let mut u: f64 = 1.0;
	for n in 0..100 {
		if n % 2 == 0 {u = 1.0}
		else {u = -1.0}
		res += u*(x.powf(2.0*(n as f64)+1.0))/fact(2.0*(n as f64)+1.0)
	}
	res
}
fn cos(a: f64) -> f64 {
	let mut x: f64 = a;
	if x > 6.2831853071795864769 {
		loop {
			if x < 6.2831853071795864769 {break}
			x -= 6.2831853071795864769;
		}
	}
	if x < -6.2831853071795864769 {
		loop {
			if x > -6.2831853071795864769 {break}
			x += 6.2831853071795864769;
		}
	}
	sin(x+1.57079632679489661923)
	}
fn tan(a: f64) -> f64 {
	let mut x: f64 = a;
	if x > 3.14159265358979323846 {
		loop {
			if x < 3.14159265358979323846 {break}
			x -= 3.14159265358979323846;
		}
	}
	if x < -3.14159265358979323846 {
		loop {
			if x > -3.14159265358979323846 {break}
			x += 3.14159265358979323846;
		}
	}
	if cos(x) == 0.0 {println!("ERROR: tan: argument not in the domain"); std::process::exit(0)}
	(sin(x))/(cos(x))}
fn arcsin(x: f64) -> f64 {
	if x > 1.0 || x < -1.0 {println!("ERROR: arcsin: argument not in the domain"); std::process::exit(0)}
  let mut res: f64 = 0.0;
  for k in 0..1000 {
    let mut prod: f64 = 1.0;
    for l in 0..k {
      prod *= (0.5 + (l as f64))/(1.0 + (l as f64))
    }
    res += (prod*(x.powf(2.0*(k as f64) + 1.0)))/(2.0*(k as f64) + 1.0)
  }
  res
}
fn arccos(x: f64) -> f64 {
	if x > 1.0 || x < -1.0 {println!("ERROR: arccos: argument not in the domain"); std::process::exit(0)}
	1.57079632679489661923 - arcsin(x)
}
fn arctan(x: f64) -> f64 {arcsin(x/(1.0 + x.powf(2.0)).powf(0.5))}
fn exp(x: f64) -> f64 {
  let e: f64 = 2.71828182845904523536028;
  e.powf(x)
}
fn ln(x: f64) -> f64 {//println!("{}", x);
	if x <= 0.0 {println!("ERROR: ln: argument not in the domain"); std::process::exit(0)}
	let mut i: f64 = -1.0;
	/*integer approximation of i:*/loop {
	  if (x - 1.0).is_sign_negative() {break}
	  if exp(i) < x && x <= exp(i + 1.0) {break}
	  i += 1.0
	}
	for _n in 0..200 {i = i - (exp(i) - x)/exp(i);}
	i
}
fn log(a: f64, b: f64) -> f64 {
	if b <= 0.0 {println!("ERROR: log: second argument not in the domain"); std::process::exit(0)}
	if a == 1.0 || a <= 0.0 {println!("ERROR: log: first argument not in the domain"); std::process::exit(0)}
	ln(b)/ln(a)
}
fn productlog(branch: f64, x: f64) -> f64 {
	if x < -exp(-1.0) {println!("ERROR: W: second argument not in the domain"); std::process::exit(0)}
	if branch != -1.0 && branch != 0.0 {println!("ERROR: W: first argument not one of -1, 0"); std::process::exit(0)}
	if branch == -1.0 && x >= 0.0 {println!("ERROR: W: for second argument >= 0, only the 0 branch is available"); std::process::exit(0)}
	let mut i: f64 = 0.0;
	if branch == -1.0 {
		i = -2.0;
		for _n in 0..200 {i = i - (i*exp(i) - x)/(exp(i)*(i+1.0));}
	}
	if branch == 0.0 {
		i = -0.9;
		loop {//integer approximation of i:
		  if i*exp(i) <= x && x < (i + 1.0)*exp(i + 1.0) {
		    break
		  }
		  i += 1.0
		}
		for _n in 0..200 {i = i - (i*exp(i) - x)/(exp(i)*(i+1.0));}
	}
	i
}
fn sqrt(x: f64) -> f64 {
	if x < 0.0 {println!("ERROR: sqrt: argument not in the domain"); std::process::exit(0)}
	x.powf(0.5)
}
fn floor(x: f64) -> f64 {
	let mut i: f64 = 0.0;
	if x.is_sign_positive() {
		loop {
			if i > x {break}
			i += 1.0
		}
		i-1.0
	}
	else {
		loop {
			if i < x {break}
			i -= 1.0
		}
		i
	}
}
fn ceil(x: f64) -> f64 {
	if x == floor(x) {return x}
	floor(x)+1.0
}
fn modulo(a: f64, b: f64) -> f64 {
	if b == 0.0 {println!("ERROR: mod: the second argument cannot be zero"); std::process::exit(0)}
	a - b*floor(a/b)
}

fn evalu8(list: Vec<Vec<String>>, lowerbound: usize, upperbound: usize) -> f64 {
	if lowerbound == upperbound {return list[lowerbound][1].parse::<f64>().unwrap()}
	let mut par_count: u32 = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "+" && par_count == 0 {return plus(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "-" && par_count == 0 {return minus(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "*" && par_count == 0 {return times(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in (lowerbound..=upperbound).rev() {
		if list[ch][1] == ")" {par_count += 1}
		if list[ch][1] == "(" {par_count -= 1}
		if list[ch][1] == "/" && par_count == 0 {return divide(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "^" && par_count == 0 {return power(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "mod" && par_count == 0 {return modulo(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "!" && par_count == 0 {return fact(evalu8(list.clone(), lowerbound, ch-1))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "sin" && par_count == 0 {return sin(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "cos" && par_count == 0 {return cos(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "tan" && par_count == 0 {return tan(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "asin" && par_count == 0 {return arcsin(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "acos" && par_count == 0 {return arccos(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "atan" && par_count == 0 {return arctan(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "exp" && par_count == 0 {return exp(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "ln" && par_count == 0 {return ln(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "log" && par_count == 0 {
			for n in lowerbound+1..=upperbound {
				if list[n][1] == "(" {par_count += 1}
				if list[n][1] == ")" {par_count -= 1}
				if list[n][1] == "," && par_count == 1 {return log(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, upperbound-1))}
			}
			println!("ERROR: log: no comma between arguments found"); std::process::exit(0)
		}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "W" && par_count == 0 {
			for n in lowerbound+1..=upperbound {
				if list[n][1] == "(" {par_count += 1}
				if list[n][1] == ")" {par_count -= 1}
				if list[n][1] == "," && par_count == 1 {return productlog(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, upperbound-1))}
			}
			println!("ERROR: W: no comma between arguments found"); std::process::exit(0)
		}
	}
	par_count = 0;
		for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "sqrt" && par_count == 0 {return sqrt(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "floor" && par_count == 0 {return floor(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "ceil" && par_count == 0 {return ceil(evalu8(list.clone(), lowerbound+1, upperbound))}
	}
	
	if list[lowerbound][1] == "(" && list[upperbound][1] == ")" {return evalu8(list.clone(), lowerbound+1, upperbound-1)}
	
	println!("Evaluation error");
	std::process::exit(0)
}

fn main() {
	loop{
  	println!("Enter an expression:");
  	let mut expr = String::new();
  	let _ = io::stdin().read_line(&mut expr);
  	let listfromstring: Vec<Vec<String>> = listovac(expr.clone());
  	println!("= {}", evalu8(listfromstring.clone(), 0, listfromstring.len()-1))
  }
}

