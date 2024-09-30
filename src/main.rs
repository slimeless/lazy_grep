mod argtype;
mod grep;
mod view;

use argtype::{ArgType, GetArgType};
use clap::Parser;
use clap_stdin::MaybeStdin;
use grep::GetContent;
use regex::Regex;
use std::{error::Error, fs::File, path::Path};
use view::display;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(default_value = "-")]
    data: MaybeStdin<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let binding = args.data.to_string();
    let new_path = Path::new(&binding);
    let regex = Regex::new(&args.pattern).unwrap();
    let matches = match new_path.get_argtype() {
        ArgType::File => {
            let file = File::open(new_path)?;
            file.get_content(&regex)?
        }
        ArgType::Directory => new_path.get_content(&regex)?,
        ArgType::Stdin => binding.get_content(&regex)?,
    };
    display(matches, regex);

    Ok(())
}
