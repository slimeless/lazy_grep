mod argtype;
mod grep;
pub mod stdin;
pub mod view;

use argtype::{ArgType, GetArgType};
use grep::GetContent;
use regex::Regex;
use std::{error::Error, fs::File, path::Path};

pub fn execute_grep(binding: String, regex: &Regex) -> Result<Vec<String>, Box<dyn Error>> {
    let new_path = Path::new(&binding);
    let matches = match new_path.get_argtype() {
        ArgType::File => {
            let file = File::open(new_path)?;
            file.get_content(regex)?
        }
        ArgType::Directory => new_path.get_content(regex)?,
        ArgType::Stdin => binding.get_content(regex)?,
    };
    Ok(matches)
}
