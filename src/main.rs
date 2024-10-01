use clap::Parser;
use regex::Regex;
use rustgrep::execute_grep;
use rustgrep::stdin::MaybeStdin;
use rustgrep::view::display;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pattern: String,
    #[arg(short, long)]
    #[clap(default_value = "-")]
    data: MaybeStdin,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let binding = args.data.to_string();
    println!("{binding}");
    let regex = Regex::new(&args.pattern).unwrap();
    let matches = execute_grep(binding, &regex)?;
    display(matches, regex);

    Ok(())
}
