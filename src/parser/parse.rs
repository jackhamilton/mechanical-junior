use std::io::{stdin, stdout, Write};

use super::{command_parser::parse_insert, execute, model::{Definition, LineCommand, SupportedLanguage}};

pub fn parse_script(script_contents: String) {
    let mut definitions: Vec<Definition> = Vec::new();
    let mut language: Option<SupportedLanguage> = None;
    for (idx, line) in script_contents.split_inclusive("\n").enumerate() {
        let command = parse_line(line, &language, &definitions, idx);
        match command {
            LineCommand::Def(definition) => definitions.push(definition),
            LineCommand::Lang(lang) => language = Some(lang),
            LineCommand::Insert(..) => execute::execute_command(command, language.as_ref().expect("Language is not defined.")),
            LineCommand::None => ()
        }
    }
}

fn parse_line(line: &str, lang: &Option<SupportedLanguage>, definitions: &Vec<Definition>, idx: usize) -> LineCommand {
    if line.starts_with("def") {
        parse_def(line, idx)
   } else if line.starts_with("lang") {
        parse_lang(line, idx)
    } else {
        parse_insert(line, lang.as_ref().expect("Need to provide language before insertion commands."), definitions, idx)
    }
}

fn parse_def(def_command: &str, idx: usize) -> LineCommand {
    let components: Vec<String> = shell_words::split(def_command).expect("Failed to split definition command");
    if components.len() != 3 {
        panic!("Malformed def command on line {idx}. Should have three components.");
    }
    print!("{}: ", components[2]);
    let _= stdout().flush();
    let mut value: String = "".to_string();
    stdin().read_line(&mut value).expect("Did not enter a correct string");
    LineCommand::Def(Definition {
        key: components[1].to_string(),
        value: value.trim().to_string()
    })
}

fn parse_lang(lang_command: &str, idx: usize) -> LineCommand {
    let parsed: Vec<&str> = lang_command.split_whitespace().collect();
    if parsed.len() != 2 {
        panic!("Incorrect format on len command on line {idx}. Should have one argument.")
    }
    if parsed[1].to_lowercase() == "swift" {
        return LineCommand::Lang(SupportedLanguage::Swift);
    }
    panic!("Language not supported: {}", parsed[1]);
}

