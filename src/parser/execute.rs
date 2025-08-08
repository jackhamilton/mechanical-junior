use std::{io::Read, io::Write, path::PathBuf};
use std::fs::File;
use walkdir::WalkDir;
use crate::Config;
use super::model::FinderOption;
use super::{model::{InsertCommand, LineCommand, SupportedLanguage}, string_utils};

pub fn execute_command(command: LineCommand, language: &SupportedLanguage) {
    match command {
        LineCommand::Insert(filename, start_finder, insert_marker_finder, command) => {
            let (file_contents, filepath) = get_file(&filename);
            let start_loc = string_utils::get_location_in_string(&file_contents, &start_finder).0.expect("No match when match expected");
            let hunk = &file_contents[start_loc..file_contents.len()];
            let insert_location_result = string_utils::get_location_in_string(hunk, &insert_marker_finder);
            apply_insert(command, filepath, file_contents, &(insert_location_result.0.expect("No match when match expected") + start_loc), insert_location_result.1);
        },
        _ => panic!("Tried to execute invalid line command")
    }
}

fn apply_insert(command: InsertCommand, filepath: PathBuf, mut contents: String, true_loc: &usize, active_options: Vec<FinderOption>) {
    let mut file = File::create(filepath).expect("Could not open file for writing");
    let matches: Vec<usize> = contents[0..*true_loc].rmatch_indices("\n").map(|item| item.0).collect();
    let mut prev_line = String::new();
    let mut idx = 0;
    let mut end_loc = *true_loc;
    while prev_line.is_empty() {
        let first_previous_line_idx = matches[idx];
        prev_line = contents[first_previous_line_idx+1..end_loc].to_string();
        end_loc = first_previous_line_idx;
        idx += 1;
    }
    let mut whitespace = String::new();
    for char in prev_line.chars() {
        if char.is_whitespace() {
            whitespace.push(char);
        } else {
            break;
        }
    }
    match command {
        InsertCommand::EnumCaseWithRaw(key, raw) => {
            let mut addendum = String::new();
            if active_options.contains(&FinderOption::IfFallbackExtraNewline) {
                addendum = "\n".to_string();
            }
            let to_write = format!("\n{whitespace}case {key} = \"{raw}\"{addendum}");
            contents.insert_str(*true_loc, &to_write);
            file.write_all(contents.as_bytes()).expect("Could not write file");
        },
    }
}

fn get_file(filename: &str) -> (String, PathBuf) {
    let config = toml_configurator::get_config::<Config>("mechanical-junior".to_string());
    let dir = match config.execution_dir.as_str() {
        "cwd" => {
            std::env::current_dir().expect("Could not lock current dir")
        }
        _ => {
            let path_str = shellexpand::tilde(&config.execution_dir).into_owned();
            let mut path = PathBuf::new();
            path.push(path_str);
            path
        }
    };
    for entry in WalkDir::new(dir) {
        let entry = entry.expect("Could not read path entry in directory.");
        if entry.path().to_str().expect("Could not convert path in directory to string.").contains(filename) {
            let mut file = std::fs::File::open(entry.path()).expect("Could not open file");
            let mut read_str = String::new();
            file.read_to_string(&mut read_str).expect("Could not read file.");
            let mut path = PathBuf::new();
            path.push(entry.path());
            return (read_str, path)
        }
    }
    panic!("File could not be found: {filename}");
}
