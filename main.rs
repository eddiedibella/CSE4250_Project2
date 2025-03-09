use std::collections::HashMap;
use std::io::{self, BufRead};

struct Program {
    lines:Vec<(132, String)>,

}
fn firstProg() {
    let stdin = io::stdin();
    let mut lines:Vec<(132, String)> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let space_index = line.find(" ")?;
            let label = line[..space_index].parse().ok()?;
            Some((label, line[space_index + 1..].to_string()))
        })
        .collect()
    lines.sort_by_key(|(label, _)| *label);
    let program = Program {lines};
    execute_program(program);

}

fn execute_program(program: Program) {


}
