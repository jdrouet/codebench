use clap::Parser;
use codebench_criterion::CriterionReader;
use codebench_env::Environment;
use codebench_reader::Reader;
use std::error::Error;
use std::path::PathBuf;

const ABOUT_COMMIT_HASH: &str = r#"Hash of the commit the results refer to.
Can be obtained using `git rev-parse HEAD`."#;
const ABOUT_BRANCH_NAME: &str = r#"Branch the results refer to.
Can be obtained using `git rev-parse --abbrev-ref HEAD`."#;

#[derive(Parser)]
struct Command {
    #[clap(long, about = ABOUT_COMMIT_HASH)]
    commit_hash: Option<String>,
    #[clap(long, about = ABOUT_BRANCH_NAME)]
    branch_name: Option<String>,
    path: PathBuf,
}

impl Command {
    /// Build the environment the results will refer to.
    fn create_environment(&self) -> Result<Environment, Box<dyn Error>> {
        Environment::parse().map(|res| {
            res.with_commit_hash(self.commit_hash.clone())
                .with_branch_name(self.branch_name.clone())
        })
    }
}

fn main() {
    let cmd = Command::parse();
    let env = cmd.create_environment();
    let reader = CriterionReader::default();
    let result = reader.evaluate(&cmd.path).unwrap();
    assert!(result.len() > 0, "no benchmarks available");
    println!("environment: {:?}", env);
    println!("{} critertion results loaded", result.len());
}
