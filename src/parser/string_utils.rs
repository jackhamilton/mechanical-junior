use regex::Regex;

use super::model::Finder;

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

pub fn get_location_in_string(str: &str, finder: Finder) -> Result<usize, FinderError> {
    match finder {
        Finder::Regex(pattern, before) => {
            let re = Regex::new(&pattern).expect("Failed to construct regex from pattern.");
            let results: Vec<&str> = re.find_iter(str).map(|m| m.as_str()).collect();
            if results.is_empty() {
                return Err(FinderError::NoMatches)
            }
            let first_match = results.first().expect("Unknown error");
            let mut loc = str.find(first_match).expect("Unknown error");
            if !before {
                loc += results.first().expect("Unknown error").len();
            }
            Ok(loc)
        },
        Finder::LineRegex(pattern, line_offset) => {
            let re = Regex::new(&pattern).expect("Failed to construct regex from pattern.");
            let results: Vec<&str> = re.find_iter(str).map(|m| m.as_str()).collect();
            if results.is_empty() {
                return Err(FinderError::NoMatches)
            }
            let first_match = results.first().expect("Unknown error");
            let loc = str.find(first_match).expect("Unknown error");
            if line_offset >= 0 {
                let hunk = &str[loc..str.len()];
                let matches: Vec<usize> = hunk.match_indices("\n").map(|item| item.0).collect();
                println!("Forward match, Pattern: {}, Matches: {:?}, line offset: {}", pattern, matches, line_offset);
                if matches.len() >= line_offset.try_into().unwrap() {
                    let index: usize = line_offset.try_into().expect("Unknown error");
                    return Ok(matches[index] + loc);
                } else {
                    return Err(FinderError::InvalidOffset)
                }
            } else {
                let hunk = &str[0..loc];
                let matches: Vec<usize> = hunk.rmatch_indices("\n").map(|item| item.0).collect();
                println!("Reverse match, Pattern: {}, Matches: {:?}, loc: {}, line_offset: {}", pattern, matches, loc, line_offset);
                if matches.len() >= (-line_offset).try_into().unwrap() {
                    let index: usize = (-line_offset).try_into().expect("Unknown error");
                    return Ok(matches[index]);
                } else {
                    return Err(FinderError::InvalidOffset)
                }
            };

        },
        Finder::LastInSectionOrFallback(finder, finder1, vec) => todo!(),
    }
}
