use std::io;
use satellite_numeric::pddl_parser::make_satellite_problem_from;
use anyhop::{process_expr_cmd_line, CmdArgs};

fn main() -> io::Result<()> {
    let cmd_args = CmdArgs::new()?;
    let verbosity:u32 = match cmd_args.num_from("v") {
        Some(n) => n,
        None => 0,
    };
    let _logger = setup_logger(verbosity);
    return process_expr_cmd_line(&make_satellite_problem_from, &cmd_args);
}

pub fn setup_logger(verbosity:u32) -> Result<(), fern::InitError> {
    let vebosity_as_object = match verbosity{
        1=> log::LevelFilter::Warn,
        2=> log::LevelFilter::Info,
        3=> log::LevelFilter::Debug,
        _=> log::LevelFilter::Info, //By default, info is a good level to have anything. Any less, and you don't get to see the results.
    };
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
        .level(vebosity_as_object)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}