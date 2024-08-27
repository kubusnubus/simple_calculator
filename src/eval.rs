use super::*;

pub fn evalu8(list: Vec<Vec<String>>, lowerbound: usize, upperbound: usize) -> f64 {
	if lowerbound == upperbound {return list[lowerbound][1].parse::<f64>().unwrap()}
	if list[lowerbound][1] == "(" && list[upperbound][1] == ")" {return evalu8(list.clone(), lowerbound+1, upperbound-1)}
	let mut par_count: u32 = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "+" && par_count == 0 {return functions::plus(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
		for ch in (lowerbound..=upperbound).rev() {
			if list[ch][1] == ")" {par_count += 1}
			if list[ch][1] == "(" {par_count -= 1}
			if list[ch][1] == "-" && par_count == 0 {return functions::minus(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
		}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if list[ch][1] == "*" && par_count == 0 {return functions::times(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in (lowerbound..=upperbound).rev() {
		if list[ch][1] == ")" {par_count += 1}
		if list[ch][1] == "(" {par_count -= 1}
		if list[ch][1] == "/" && par_count == 0 {return functions::divide(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
	}
	par_count = 0;
	for ch in lowerbound..=upperbound {
		if list[ch][1] == "(" {par_count += 1}
		if list[ch][1] == ")" {par_count -= 1}
		if par_count == 0 {
			if list[ch][0] == "binary_fn" {
				if list[ch][1] == "^" {return functions::power(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
			}
			if list[ch][0] == "unary_fn" {
				if list[ch][1] == "mod" {return functions::modulo(evalu8(list.clone(), lowerbound, ch-1), evalu8(list, ch+1, upperbound))}
				if list[ch][1] == "!" {return functions::fact(evalu8(list.clone(), lowerbound, ch-1))}
				if list[ch][1] == "sin" {return functions::sin(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "cos" {return functions::cos(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "tan" {return functions::tan(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "asin" {return functions::arcsin(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "acos" {return functions::arccos(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "atan" {return functions::arctan(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "exp" {return functions::exp(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "ln" {return functions::ln(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "log" {
					for n in lowerbound+1..=upperbound {
						if list[n][1] == "(" {par_count += 1}
						if list[n][1] == ")" {par_count -= 1}
						if list[n][1] == "," && par_count == 1 {return functions::log(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, upperbound-1))}
					}
					println!("ERROR: log: no comma between arguments found"); std::process::exit(0)
				}
				if list[ch][1] == "W" {
					for n in lowerbound+1..=upperbound {
						if list[n][1] == "(" {par_count += 1}
						if list[n][1] == ")" {par_count -= 1}
						if list[n][1] == "," && par_count == 1 {return functions::productlog(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, upperbound-1))}
					}
					println!("ERROR: W: no comma between arguments found"); std::process::exit(0)
				}
				if list[ch][1] == "sqrt" {return functions::sqrt(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "floor" {return functions::floor(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "ceil" {return functions::ceil(evalu8(list.clone(), lowerbound+1, upperbound))}
				if list[ch][1] == "sum" {
					for n in lowerbound+1..=upperbound {
						if list[n][1] == "(" {par_count += 1}
						if list[n][1] == ")" {par_count -= 1}
						if list[n][1] == "," && par_count == 1 {
							for k in n+1..=upperbound {
								if list[k][1] == "(" {par_count += 1}
								if list[k][1] == ")" {par_count -= 1}
								if list[k][1] == "," && par_count == 1 {return functions::sum(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, k-1), k+1, upperbound-1, list)}
							}
							println!("ERROR: sum: only one comma between arguments found"); std::process::exit(0)
						}
					}
					println!("ERROR: sum: no comma between arguments found"); std::process::exit(0)
				}
				if list[ch][1] == "prod" {
					for n in lowerbound+1..=upperbound {
						if list[n][1] == "(" {par_count += 1}
						if list[n][1] == ")" {par_count -= 1}
						if list[n][1] == "," && par_count == 1 {
							for k in n+1..=upperbound {
								if list[k][1] == "(" {par_count += 1}
								if list[k][1] == ")" {par_count -= 1}
								if list[k][1] == "," && par_count == 1 {return functions::prod(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, k-1), k+1, upperbound-1, list)}
							}
							println!("ERROR: prod: only one comma between arguments found"); std::process::exit(0)
						}
					}
					println!("ERROR: prod: no comma between arguments found"); std::process::exit(0)
				}
				if list[ch][1] == "int" {
					for n in lowerbound+1..=upperbound {
						if list[n][1] == "(" {par_count += 1}
						if list[n][1] == ")" {par_count -= 1}
						if list[n][1] == "," && par_count == 1 {
							for k in n+1..=upperbound {
								if list[k][1] == "(" {par_count += 1}
								if list[k][1] == ")" {par_count -= 1}
								if list[k][1] == "," && par_count == 1 {return functions::int(evalu8(list.clone(), lowerbound+2, n-1), evalu8(list.clone(), n+1, k-1), k+1, upperbound-1, list)}
							}
							println!("ERROR: int: only one comma between arguments found"); std::process::exit(0)
						}
					}
					println!("ERROR: int: no comma between arguments found"); std::process::exit(0)
				}
				if list[ch][1] == "der" {
					for n in lowerbound+1..=upperbound {
						if list[n][1] == "(" {par_count += 1}
						if list[n][1] == ")" {par_count -= 1}
						if list[n][1] == "," && par_count == 1 {return functions::der(evalu8(list.clone(), lowerbound+2, n-1), n+1, upperbound-1, list)}
					}
					println!("ERROR: der: no comma between arguments found"); std::process::exit(0)
				}
			}
		}
	}
	println!("Evaluation error");
	std::process::exit(0)
}
