mod argtype;
mod grep;
mod sort;
pub mod stdin;
pub mod view;

use argtype::{ArgType, GetArgType};
use grep::GetContent;
use regex::Regex;
use sort::sort_dir_vec;
use std::{error::Error, fs::File, path::Path};

pub fn execute_grep(
    binding: String,
    regex: &Regex,
    excludes: &mut Vec<Regex>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let new_path = Path::new(&binding);
    let matches = match new_path.get_argtype() {
        ArgType::File => {
            let file = File::open(new_path)?;
            file.get_content(regex)?
        }
        ArgType::Directory => {
            let mut res = new_path.get_content(regex)?;
            if !excludes.is_empty() {
                res = sort_by_excludes(&mut res, excludes);
            };
            res
        }
        ArgType::Stdin => binding.get_content(regex)?,
    };
    Ok(matches)
}

fn sort_by_excludes(matches: &mut Vec<String>, excludes: &mut Vec<Regex>) -> Vec<String> {
    let res = matches;
    while let Some(regex) = excludes.pop() {
        sort_dir_vec(regex, res);
    }
    res.to_vec()
}
