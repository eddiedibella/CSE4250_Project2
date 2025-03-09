use std::collections::BTreeMap;
use std::io:{self, BufRead};

enum statement {
    let(char, String),
    Print(String),
    Println(String),
    if GoTo(String, usize),
}

fn parse_arithmatic_expr(expr: &str, vars: BTreeMap<char, i32>) -> i32 {


}

