use std::io;
use satellite_numeric::pddl_parser::make_satellite_problem_from;
use anyhop::process_expr_cmd_line;
fn main() -> io::Result<()> {
    return process_expr_cmd_line(&make_satellite_problem_from);
}

