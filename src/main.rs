use std::io;
use satellite_numeric::pddl_parser::make_satellite_problem_from;
use anyhop::process_expr_cmd_line;
fn main() -> io::Result<()> {
    let logger = setup_logger();
    return process_expr_cmd_line(&make_satellite_problem_from);
}

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}]{}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}