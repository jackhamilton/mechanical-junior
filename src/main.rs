use serde::Serialize;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

mod parser;

#[derive(Serialize, Deserialize)]
struct Config {
    script_dir: String,
    execution_dir: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            script_dir: "~/.config/mechanical-junior/scripts".to_string(),
            execution_dir: "cwd".to_string()
        }
    }
}

struct Script {
    name: String,
    path: PathBuf
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments. Please provide a script to run.");
    }
    let script_name = &args[1];
    let config = toml_configurator::get_config::<Config>("mechanical-junior".to_string());
    let scripts = load_scripts(&config.script_dir);
    for script in scripts {
        if script.name.starts_with(script_name) {
            // Run the first and only the first script matching input.
            run_script(script);
            return
        }
    }
}

fn load_scripts(dir: &str) -> Vec<Script> {
    let paths = fs::read_dir(shellexpand::tilde(dir).as_ref()).expect("Could not read script directory.");
    let mut output_paths = Vec::<Script>::new();
    for path in paths {
        let path = path.expect("Could not read script path.");
        output_paths.push(
            Script {
                name: path.file_name().to_str().expect("Could not convert filename to string").to_string(),
                path: path.path()
            }
        );
    }
    output_paths
}

fn run_script(script: Script) {
    let mut contents = fs::File::open(script.path).expect("Could not open specified script");
    let mut string_contents = String::new();
    contents.read_to_string(&mut string_contents).expect("Could not read script as utf8");
    parser::parse::parse_script(string_contents);
}
