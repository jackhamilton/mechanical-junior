use std::{io::Read, path::PathBuf};
use walkdir::WalkDir;
use crate::Config;
use super::{model::{LineCommand, SupportedLanguage}, string_utils};

pub fn execute_command(command: LineCommand, language: &SupportedLanguage) {
    match command {
        LineCommand::Insert(filename, start_finder, insert_marker_finder, command) => {
            let file_contents = get_file(&filename);
            let start_loc = string_utils::get_location_in_string(&file_contents, start_finder).expect("No match when match expected");
            println!("Match {}", start_loc);
        },
        _ => panic!("Tried to execute invalid line command")
    }
}

fn get_file(filename: &str) -> String {
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
            return read_str
        }
    }
    panic!("File could not be found: {filename}");
}
