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
        if let Some(matched) = re.find(self) {
            let range = matched.range();
            let before = &self[..range.start];
            let colored_text = &self[range.start..range.end];
            let after = &self[range.end..];

            let mut result = String::with_capacity(self.len() + 10);
            result.push_str(before);
            result.push_str(&colored_text.bold().red().to_string());
            result.push_str(after);

            result
        } else {
            self.clone()
        }
    }
}
