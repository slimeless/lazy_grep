use core::panic;

use regex::Regex;

pub fn sort_dir_vec(re: Regex, vec: &mut Vec<String>) {
    vec.retain(|item| {
        let index = match item.find(":") {
            Some(i) => i,
            None => {
                panic!("failed to get : when sorting vector")
            }
        };
        !re.is_match(&item[..index])
    });
    *vec = vec.to_vec();
}
