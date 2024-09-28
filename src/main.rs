mod grep;
use clap::Parser;
use clap_stdin::MaybeStdin;
use grep::GetContent;
use regex::Regex;
use std::{error::Error, fs::File, path::Path};

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
    let matches = if new_path.exists() && new_path.is_file() {
        let file = File::open(new_path)?;
        println!("file");
        file.get_content(regex)?
    } else if new_path.exists() {
        println!("dir");
        new_path.get_content(regex)?
    } else {
        println!("raw text/stdin");
        args.data.to_string().get_content(regex)?
    };
    print_result(matches);

    Ok(())
}
fn print_result(res: Vec<String>) {
    println!("count of pattern: {}", res.len());
    for line in res {
        println!("{line}")
    }
}
