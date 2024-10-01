use clap::Parser;
use regex::Regex;
use rustgrep::execute_grep;
use rustgrep::stdin::MaybeStdin;
use rustgrep::view::display;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(default_value = "-")]
    data: MaybeStdin,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let binding = args.data.to_string();
    let regex = Regex::new(&args.pattern).unwrap();
    let matches = execute_grep(binding, &regex)?;
    display(matches, regex);

    Ok(())
}
