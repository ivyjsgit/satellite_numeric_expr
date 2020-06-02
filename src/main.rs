use std::{env, io, fs};
use blocks_world::pddl_parser::make_block_problem_from;
use anyhop::BacktrackPreference::*;
use anyhop::BacktrackStrategy::*;
use anyhop::AnytimePlannerBuilder;
use std::fs::File;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut results = anyhop::summary_csv_header();
    let mut limit_ms = None;
    let mut args_iter = env::args().skip(1).peekable();
    while args_iter.peek().map_or(false, |s| s.starts_with("-")) {
        match args_iter.next().unwrap().as_str() {
            "-5s" => limit_ms = Some(5000),
            tag => println!("Unrecognized argument: {}",tag)
        }
    }
    for file in args_iter {
        if file.ends_with("*") {
            let mut no_star = file.clone();
            no_star.pop();
            for entry in fs::read_dir(".")? {
                let entry = entry?;
                let entry = entry.file_name();
                let entry = entry.to_str();
                let entry_name = entry.unwrap();
                if entry_name.starts_with(no_star.as_str()) {
                    assess_file(entry_name, &mut results, limit_ms)?;
                }
            }
        } else {
            assess_file(file.as_str(), &mut results, limit_ms)?;
        }
    }
    let mut output = File::create("results.csv")?;
    write!(output, "{}", results.as_str())?;
    Ok(())
}

fn assess_file(file: &str, results: &mut String, limit_ms: Option<u128>) -> io::Result<()> {
    println!("Running {}", file);
    let (start, goal) = make_block_problem_from(file)?;
    for strategy in vec![Alternate(LeastRecent), Steady(LeastRecent), Steady(MostRecent)] {
        for apply_cutoff in vec![true, false] {
            let outcome = AnytimePlannerBuilder::state_goal(&start, &goal)
                .apply_cutoff(apply_cutoff)
                .strategy(strategy)
                .possible_time_limit_ms(limit_ms)
                .verbose(1)
                .construct();
            let label = format!("o_{}_{:?}_{}", desuffix(file), strategy, if apply_cutoff {"cutoff"} else {"no_cutoff"}).replace(")", "_").replace("(", "_");
            let row = outcome.summary_csv_row(label.as_str());
            print!("{}", row);
            results.push_str(row.as_str());
            let mut output = File::create(format!("{}.csv", label))?;
            write!(output, "{}", outcome.instance_csv())?;
        }
    }
    Ok(())
}

pub fn desuffix(filename: &str) -> String {
    filename.split(".").next().unwrap().to_string()
}