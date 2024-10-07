use owo_colors::{OwoColorize, Style};
use regex::Regex;

pub fn display(matches: Vec<String>, _re: Regex) {
    let green_style: Style = Style::new().bold().green();
    let count = format!("count of matches: {}", matches.len());
    println!("{}", count.style(green_style));
    for line in matches {
        println!("{line}")
    }
}

pub trait Highlight {
    fn highlight(&self, re: &Regex) -> String;
}

impl Highlight for String {
    fn highlight(&self, re: &Regex) -> String {
        let mut result = String::new();
        let mut last_end = 0;

        for matched in re.find_iter(self) {
            result.push_str(&self[last_end..matched.start()]);
            result.push_str(&matched.as_str().bold().red().to_string());
            last_end = matched.end();
        }

        result.push_str(&self[last_end..]);
        result
    }
}
