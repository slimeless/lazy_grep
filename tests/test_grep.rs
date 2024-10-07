use std::error::Error;

use regex::Regex;
use rustgrep::execute_grep;

#[test]
fn test_email() -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$")?;
    let binding = "john.doe@example.com\nalice.smith123@sub.domain.com\nuser+tag@domain.co.uk\nfirst_last@domain.org\nname@domain123.com\n\ninvalid-email@.com\nuser@domain..com\n@missingusername.com\nusername@domain.c\nuser@domain@another.com\n";
    let result = execute_grep(binding.to_string(), &re, &mut Vec::new())?;
    assert_eq!(result.len(), 6);
    Ok(())
}
