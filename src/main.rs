mod grep;
use clap::Parser;
use grep::GetContent;
use regex::Regex;
use std::{fs::File, io::Result, path::Path};

#[derive(Parser)]
struct Cli {
    pattern: String,
    data: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let new_path = Path::new(&args.data);
    let regex = Regex::new(&args.pattern).unwrap();
    let matches = if new_path.exists() && new_path.is_file() {
        let file = File::open(new_path)?;
        just_grep(file, regex)
    } else {
        just_grep(args.data, regex)
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

fn just_grep<T: GetContent>(data: T, re: Regex) -> Vec<String> {
    let mut string_mathces = Vec::new();
    get_matches(data, re, &mut string_mathces);
    string_mathces
}
fn get_matches<T: GetContent>(content: T, re: Regex, res_vec: &mut Vec<String>) {
    for line in content.get_content() {
        if re.is_match(&line) {
            //println!("{line} {}", line.len());
            res_vec.push(line);
        }
    }
}
