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
    #[clap(short, long)]
    exclude: Option<Vec<Regex>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let binding = args.data.to_string();
    let regex = Regex::new(&args.pattern).unwrap();
    let mut excludes = args.exclude.unwrap_or_default();
    println!("{excludes:?}");
    let matches = execute_grep(binding, &regex, &mut excludes)?;
    display(matches, regex);

    Ok(())
}
