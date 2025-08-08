use regex::Regex;

use super::model::{Finder, FinderOption};

pub fn remove_paren_contents(str: &str) -> String {
    let first = str.find("(").unwrap_or_else(|| panic!("No open paren. parsing string {str}"));
    let last = str.rfind(")").unwrap_or_else(|| panic!("No close paren. parsing string {str}"));
    let mut output = str.to_string()[0..first].to_string();
    output.push_str(&str.to_string()[last + 1..str.len()]);
    output
}

pub fn paren_contents(str: &str) -> String {
    let first = str.find("(").unwrap_or_else(|| panic!("No open paren. parsing string {str}"));
    let last = str.rfind(")").unwrap_or_else(|| panic!("No close paren. parsing string {str}"));
    (str.to_string()[first + 1..last]).to_string()
}

#[derive(Debug)]
pub enum FinderError{
    Generic,
    NoMatches,
    InvalidOffset
}

pub fn get_location_in_string(str: &str, finder: &Finder) -> (Result<usize, FinderError>, Vec<FinderOption>) {
    match finder {
        Finder::Regex(pattern, before) => {
            let re = Regex::new(&pattern).expect("Failed to construct regex from pattern.");
            let results: Vec<&str> = re.find_iter(str).map(|m| m.as_str()).collect();
            if results.is_empty() {
                return (Err(FinderError::NoMatches), vec![])
            }
            let first_match = results.first().expect("Unknown error");
            let mut loc = str.find(first_match).expect("Unknown error");
            if !before {
                loc += results.first().expect("Unknown error").len();
            }
            (Ok(loc), vec![])
        },
        Finder::LineRegex(pattern, line_offset) => {
            let re = Regex::new(&pattern).expect("Failed to construct regex from pattern.");
            let results: Vec<&str> = re.find_iter(str).map(|m| m.as_str()).collect();
            if results.is_empty() {
                return (Err(FinderError::NoMatches), vec![])
            }
            let first_match = results.first().expect("Unknown error");
            let loc = str.find(first_match).expect("Unknown error");
            let line_offset: i32 = *line_offset;
            if line_offset >= 0 {
                let hunk = &str[loc..str.len()];
                let matches: Vec<usize> = hunk.match_indices("\n").map(|item| item.0).collect();

                if matches.len() >= line_offset.try_into().unwrap() {
                    let index: usize = line_offset.try_into().expect("Unknown error");
                    return (Ok(matches[index] + loc), vec![])
                } else {
                    return (Err(FinderError::InvalidOffset), vec![])
                }
            } else {
                let hunk = &str[0..loc];
                let matches: Vec<usize> = hunk.rmatch_indices("\n").map(|item| item.0).collect();
                if matches.len() >= (-line_offset).try_into().unwrap() {
                    let index: usize = (-line_offset).try_into().expect("Unknown error");
                    return (Ok(matches[index]), vec![])
                } else {
                    return (Err(FinderError::InvalidOffset), vec![])
                }
            };

        },
        Finder::LastInSectionOrFallback(finder, finder1, vec) => {
            let finder = finder.as_ref();
            match get_location_in_string(str, &finder) {
                (Ok(item), opts) => (Ok(item), vec![]),
                (Err(_), opts) => {
                    let out = get_location_in_string(str, &finder1);
                    (out.0, vec.clone())
                }
            }
        },
    }
}
