pub fn listovac(expr_temp: String) -> Vec<Vec<String>> {
    let mut expr = expr_temp;
    expr = expr.replace(" ", "");
    expr = expr.replace("\n", "");
    expr = expr.replace("\r", "");
    //separating:
    /*projíždím charactery expr zleva, přiřadím každému charu typ (a zkontroluji, zda je podporován), nakonec sloučím typy num a rightside_fn/const, které jsou vedle sebe
    a na závěr rozdělím rightside_fn/const na rightside_fn a const a zkontroluji, jestli jsou sloučené funkce/konstanty podporovány)*/
    let list_of_expr_chars: Vec<char> = expr.chars().collect();
    let mut list: Vec<Vec<String>> = vec![];
    //0->num 1->() 2->binary_fn 3->chars (unary_fn/const) 4->comma 5->k (index_var) 6->x (int_var)
    let types_of_chars: Vec<Vec<String>> = vec![
        vec![
            "0".to_string(),
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            ".".to_string(),
        ],
        vec!["(".to_string(), ")".to_string()],
        vec![
            "+".to_string(),
            "-".to_string(),
            "*".to_string(),
            "/".to_string(),
            "^".to_string(),
        ],
        vec![
            "!".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
            "f".to_string(),
            "g".to_string(),
            "h".to_string(),
            "i".to_string(),
            "j".to_string(),
            /*"k".to_string(),*/ "l".to_string(),
            "m".to_string(),
            "n".to_string(),
            "o".to_string(),
            "p".to_string(),
            "q".to_string(),
            "r".to_string(),
            "s".to_string(),
            "t".to_string(),
            "u".to_string(),
            "v".to_string(),
            "w".to_string(),
            /*"x".to_string(),*/ "y".to_string(),
            "z".to_string(),
            "W".to_string(),
        ],
        vec![",".to_string()],
        vec!["k".to_string()],
        vec!["x".to_string()],
    ];
    let mut last_type: usize = 7;
    for ch in 0..expr.len() {
        'outer: for k in 0..types_of_chars.len() {
            for l in 0..types_of_chars[k].len() {
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 0 {
                    if last_type == 0 {
                        let lastindex: usize = list.len() - 1;
                        let lastlastindex: usize = list[lastindex].len() - 1;
                        list[lastindex][lastlastindex].push(list_of_expr_chars[ch]);
                        break 'outer;
                    } else {
                        list.push(vec!["number".to_string(), list_of_expr_chars[ch].to_string()]);
                        last_type = 0;
                        break 'outer;
                    }
                }
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 1 {
                    list.push(vec!["par".to_string(), list_of_expr_chars[ch].to_string()]);
                    last_type = 1;
                    break 'outer;
                }
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 2 {
                    list.push(vec!["binary_fn".to_string(), list_of_expr_chars[ch].to_string()]);
                    last_type = 2;
                    break 'outer;
                }
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 3 {
                    if last_type == 3 {
                        let lastindex: usize = list.len() - 1;
                        let lastlastindex: usize = list[lastindex].len() - 1;
                        list[lastindex][lastlastindex].push(list_of_expr_chars[ch]);
                        break 'outer;
                    } else {
                        list.push(vec!["unary_fn/const".to_string(), list_of_expr_chars[ch].to_string()]);
                        last_type = 3;
                        break 'outer;
                    }
                }
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 4 {
                    list.push(vec!["comma".to_string(), list_of_expr_chars[ch].to_string()]);
                    last_type = 4;
                    break 'outer;
                }
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 5 {
                    list.push(vec!["index_var".to_string(), list_of_expr_chars[ch].to_string()]);
                    last_type = 5;
                    break 'outer;
                }
                if list_of_expr_chars[ch].to_string() == types_of_chars[k][l] && k == 6 {
                    list.push(vec!["int_var".to_string(), list_of_expr_chars[ch].to_string()]);
                    last_type = 6;
                    break 'outer;
                }
                if list_of_expr_chars[ch].to_string() != types_of_chars[k][l]
                    && k == 6
                    && l == types_of_chars[6].len() - 1
                {
                    println!("ERROR: unsupported character: {}", list_of_expr_chars[ch]);
                    std::process::exit(0)
                }
            }
        }
    }
    let fn_list: Vec<String> = vec![
        "der".to_string(),
        "int".to_string(),
        "prod".to_string(),
        "sum".to_string(),
        "ceil".to_string(),
        "floor".to_string(),
        "mod".to_string(),
        "!".to_string(),
        "sqrt".to_string(),
        "ln".to_string(),
        "exp".to_string(),
        "log".to_string(),
        "sin".to_string(),
        "cos".to_string(),
        "tan".to_string(),
        "asin".to_string(),
        "acos".to_string(),
        "atan".to_string(),
        "W".to_string(),
    ];
    let const_list: Vec<String> = vec!["pi".to_string(), "e".to_string()];
    for ch in 0..list.len() {
        if list[ch][0] == "unary_fn/const" {
            let mut fn_or_const: bool = false;
            for k in 0..fn_list.len() {
                if list[ch][1] == fn_list[k] {
                    list[ch][0] = "unary_fn".to_string();
                    fn_or_const = true;
                    break;
                }
            }
            for k in 0..const_list.len() {
                if list[ch][1] == const_list[k] {
                    list[ch][0] = "const".to_string();
                    fn_or_const = true;
                    break;
                }
            }
            if fn_or_const == false {
                println!("ERROR: unsupported function/constant: {}", list[ch][1]);
                std::process::exit(0)
            }
        }
    }
    let mut par_count: i32 = 0;
    for ch in 0..list.len() {
        if list[ch][1] == "pi" {
            list[ch][1] = "3.14159265358979323846".to_string();
            list[ch][0] = "number".to_string()
        }
        if list[ch][1] == "e" {
            list[ch][1] = "2.71828182845904523536".to_string();
            list[ch][0] = "number".to_string()
        }
        if list[ch][1] == "-" && ch == 0 {
            let mut minuslist: Vec<Vec<String>> = vec![vec!["number".to_string(), "0".to_string()]]; //unary minus => binary minus
            for h in 0..list.len() {
                minuslist.push(list[h].clone())
            }
            list = minuslist
        }
        if list[ch][1] == "-" && (list[ch - 1][1] == "(" || list[ch - 1][1] == ",") {
            let mut minuslist: Vec<Vec<String>> = vec![];
            for h in 0..ch {
                minuslist.push(list[h].clone())
            }
            minuslist.push(vec!["number".to_string(), "0".to_string()]);
            for h in ch..list.len() {
                minuslist.push(list[h].clone())
            }
            list = minuslist
        }
    }
    for ch in 0..list.len() - 1 {
        //implicit multiplication
        if list[ch][0] == "number" && list[ch + 1][0] == "number" {
            let mut implicitlist: Vec<Vec<String>> = vec![];
            for h in 0..=ch {
                implicitlist.push(list[h].clone())
            }
            implicitlist.push(vec!["binary_fn".to_string(), "*".to_string()]);
            for h in ch + 1..list.len() {
                implicitlist.push(list[h].clone())
            }
            list = implicitlist
        }
    }
    for ch in 0..list.len() {
        if list[ch][1] == "(" {
            par_count += 1
        }
        if list[ch][1] == ")" {
            par_count -= 1
        }
    }
    if par_count < 0 {
        println!("ERROR: not enough opening parentheses");
        std::process::exit(0)
    }
    if par_count > 0 {
        println!("ERROR: not enough closing parentheses");
        std::process::exit(0)
    }
    for ch in 0..list.len() {
        if list[ch][0] == "number" {
            break;
        }
        if ch == list.len() - 1 {
            println!("ERROR: expression does not contain any numbers");
            std::process::exit(0)
        }
    }
    //println!("{:?}", list);
    list
}
