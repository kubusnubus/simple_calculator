use super::*;

pub fn plus(a: f64, b: f64) -> f64 {a + b}
pub fn minus(a: f64, b: f64) -> f64 {a - b}
pub fn times(a: f64, b: f64) -> f64 {a * b}
pub fn divide(a: f64, b: f64) -> f64 {
	if b == 0.0 {println!("ERROR: division by zero"); std::process::exit(0)}
	a / b
	}
pub fn power(a: f64, b: f64) -> f64 {a.powf(b)}
pub fn fact(n: f64) -> f64 {
	for k in 0..=21 {
		if n == k as f64 {break}
		if k == 21 {println!("ERROR: factorial: argument is not one of 0..=21 (range limited for the sake of accuracy)"); std::process::exit(0)}
	}
	if n == 0.0 {return 1.0}
	n*fact(n-1.0)
}
pub fn sin(a: f64) -> f64 {
	let x: f64 = modulo(a, 6.2831853071795864769);
	let mut res: f64 = 0.0;
	if x > 3.1415926535897932385 {return -sin(x - 3.1415926535897932385)}
	if x > 1.57079632679489661923 {return sin(3.1415926535897932385 - x)}
	for n in 0..=10 {
		if n % 2 == 0 {res += (x.powf(2.0*(n as f64)+1.0))/fact(2.0*(n as f64)+1.0)}
		else {res -= (x.powf(2.0*(n as f64)+1.0))/fact(2.0*(n as f64)+1.0)}
	}
	res
}
pub fn cos(x: f64) -> f64 {sin(x+1.57079632679489661923)}
pub fn tan(x: f64) -> f64 {
	if cos(x) == 0.0 {println!("ERROR: tan: argument not in the domain"); std::process::exit(0)}
	sin(x)/cos(x)}
pub fn arcsin(x: f64) -> f64 {
	if x > 1.0 || x < -1.0 {println!("ERROR: arcsin: argument not in the domain"); std::process::exit(0)}
  let mut res: f64 = 0.0;
  for k in 0..10000 {
    let mut prod: f64 = 1.0;
    for l in 0..k {
      prod *= (0.5 + (l as f64))/(1.0 + (l as f64))
    }
    res += (prod*(x.powf(2.0*(k as f64) + 1.0)))/(2.0*(k as f64) + 1.0)
  }
  res
}
pub fn arccos(x: f64) -> f64 {
	if x > 1.0 || x < -1.0 {println!("ERROR: arccos: argument not in the domain"); std::process::exit(0)}
	1.57079632679489661923 - arcsin(x)
}
pub fn arctan(x: f64) -> f64 {arcsin(x/(1.0 + x.powf(2.0)).powf(0.5))}
pub fn exp(x: f64) -> f64 {
  let e: f64 = 2.71828182845904523536028;
  e.powf(x)
}
pub fn ln(x: f64) -> f64 {//println!("{}", x);
	if x <= 0.0 {println!("ERROR: ln: argument not in the domain"); std::process::exit(0)}
	let mut i: f64 = -1.0;
	/*integer approximation of i:*/loop {
	  if (x - 1.0).is_sign_negative() {break}
	  if exp(i) < x && x <= exp(i + 1.0) {break}
	  i += 1.0
	}
	for _ in 0..200 {i = i - (exp(i) - x)/exp(i);}
	i
}
pub fn log(a: f64, b: f64) -> f64 {
	if b <= 0.0 {println!("ERROR: log: second argument not in the domain"); std::process::exit(0)}
	if a == 1.0 || a <= 0.0 {println!("ERROR: log: first argument not in the domain"); std::process::exit(0)}
	ln(b)/ln(a)
}
pub fn productlog(branch: f64, x: f64) -> f64 {
	if x < -exp(-1.0) {println!("ERROR: W: second argument not in the domain"); std::process::exit(0)}
	if branch != -1.0 && branch != 0.0 {println!("ERROR: W: first argument not one of -1, 0"); std::process::exit(0)}
	if branch == -1.0 && x >= 0.0 {println!("ERROR: W: for second argument >= 0, only the 0 branch is available"); std::process::exit(0)}
	let mut i: f64 = 0.0;
	if branch == -1.0 {
		i = -2.0;
		for _ in 0..200 {i = i - (i*exp(i) - x)/(exp(i)*(i+1.0));}
	}
	if branch == 0.0 {
		i = -0.9;
		loop {//integer approximation of i:
		  if i*exp(i) <= x && x < (i + 1.0)*exp(i + 1.0) {
		    break
		  }
		  i += 1.0
		}
		for _ in 0..200 {i = i - (i*exp(i) - x)/(exp(i)*(i+1.0));}
	}
	i
}
pub fn sqrt(x: f64) -> f64 {
	if x < 0.0 {println!("ERROR: sqrt: argument not in the domain"); std::process::exit(0)}
	x.powf(0.5)
}
pub fn floor(x: f64) -> f64 {
	x.floor()
}
pub fn ceil(x: f64) -> f64 {
	if x == floor(x) {return x}
	floor(x) + 1.0
}
pub fn modulo(a: f64, b: f64) -> f64 {
	if b == 0.0 {println!("ERROR: mod: the second argument cannot be zero"); std::process::exit(0)}
	a - b*floor(a/b)
}
pub fn sum(start_tmp: f64, stop_tmp: f64, exprstart: usize, exprstop: usize, list_tmp: Vec<Vec<String>>) -> f64 {
	if floor(start_tmp) != start_tmp {println!("ERROR: sum: the first argument must be an integer"); std::process::exit(0)}
	if floor(stop_tmp) != stop_tmp {println!("ERROR: sum: the second argument must be an integer"); std::process::exit(0)}
	let start: usize = start_tmp.floor() as usize;
	let stop: usize = stop_tmp.floor() as usize;
	let mut res: f64 = 0.0;
	for n in start..=stop {
		let mut list: Vec<Vec<String>> = list_tmp.clone();
		for ch in exprstart..=exprstop {if list[ch][1] == "k" {list[ch][1] = n.to_string()}}
		res += eval::evalu8(list.clone(), exprstart, exprstop);
	}
	res
}
pub fn prod(start_tmp: f64, stop_tmp: f64, exprstart: usize, exprstop: usize, list_tmp: Vec<Vec<String>>) -> f64 {
	if floor(start_tmp) != start_tmp {println!("ERROR: prod: the first argument must be an integer"); std::process::exit(0)}
	if floor(stop_tmp) != stop_tmp {println!("ERROR: prod: the second argument must be an integer"); std::process::exit(0)}
	let start: usize = start_tmp.floor() as usize;
	let stop: usize = stop_tmp.floor() as usize;
	let mut res: f64 = 1.0;
	for n in start..=stop {
		let mut list: Vec<Vec<String>> = list_tmp.clone();
		for ch in exprstart..=exprstop {if list[ch][1] == "k" {list[ch][1] = n.to_string()}}
		res *= eval::evalu8(list.clone(), exprstart, exprstop);
	}
	res
}
pub fn int(a: f64, b: f64, exprstart: usize, exprstop_tmp: usize, list_tmp: Vec<Vec<String>>) -> f64 {
	let mut list: Vec<Vec<String>> = list_tmp.clone();
	let n: f64 = 500000.0;
	let mut exprstop: usize = exprstop_tmp.clone();
	'outer: loop {
		for ch in exprstart..=exprstop {
			if list[ch][1] == "x" {
				let mut list_tmp_tmp = vec![];
				for i in 0..ch {list_tmp_tmp.push(list[i].clone())}
				list_tmp_tmp.push(vec!["".to_string(), "(".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), a.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "+".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "k".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "*".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "(".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), b.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "-".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), a.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), ")".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "/".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), n.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), ")".to_string()]);
				for i in ch+1..list.len() {list_tmp_tmp.push(list[i].clone())}
				list = list_tmp_tmp;
				exprstop += 12
			}
			if ch == exprstop {break 'outer}
		}
	}
	((b-a)/n)*sum(0.0, n, exprstart, exprstop, list)
}
pub fn der(a: f64, exprstart: usize, exprstop: usize, list_tmp: Vec<Vec<String>>) -> f64 {
	let mut list1: Vec<Vec<String>> = list_tmp.clone();
	let mut list2: Vec<Vec<String>> = list_tmp.clone();
	let mut exprstop1: usize = exprstop.clone();
	let n: f64 = 10000000.0;
	'outer: loop {
		for ch in exprstart..=exprstop1 {
			if list1[ch][1] == "x" {
				let mut list_tmp_tmp = vec![];
				for i in 0..ch {list_tmp_tmp.push(list1[i].clone())}
				list_tmp_tmp.push(vec!["".to_string(), "(".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "(".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), n.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "*".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), a.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "+".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "1".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), ")".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), "/".to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), n.to_string()]);
				list_tmp_tmp.push(vec!["".to_string(), ")".to_string()]);
				for i in ch+1..list1.len() {list_tmp_tmp.push(list1[i].clone())}
				list1 = list_tmp_tmp;
				exprstop1 += 10
			}
			if ch == exprstop1 {break 'outer}
		}
	}
	for ch in exprstart..=exprstop {
		if list2[ch][1] == "x" {
			let mut list_tmp_tmp = vec![];
			for i in 0..ch {list_tmp_tmp.push(list2[i].clone())}
			list_tmp_tmp.push(vec!["".to_string(), a.to_string()]);
			for i in ch+1..list2.len() {list_tmp_tmp.push(list2[i].clone())}
			list2 = list_tmp_tmp;
		}
	}
	n*(eval::evalu8(list1, exprstart, exprstop1) - eval::evalu8(list2, exprstart, exprstop))
}
