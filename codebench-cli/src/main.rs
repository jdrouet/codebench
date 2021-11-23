use clap::Parser;
use codebench_criterion::CriterionReader;
use codebench_reader::Reader;
use std::path::PathBuf;

#[derive(Parser)]
struct Command {
    path: PathBuf,
}

fn main() {
    let cmd = Command::parse();
    let reader = CriterionReader::default();
    let result = reader.evaluate(&cmd.path).unwrap();
    assert!(result.len() > 0, "no benchmarks available");
    println!("{} critertion results loaded", result.len());
}
