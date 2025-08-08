use convert_case::Casing;

use super::{model::*, string_utils};

pub fn parse_insert(insert_command: &str, lang: &SupportedLanguage, definitions: &Vec<Definition>, idx: usize) -> LineCommand {
    let components: Vec<&str> = insert_command.split(";").collect();
    if components.len() < 3 {
        panic!("Not enough arguments to insertion command on line {idx}");
    }
    let file = components[0];
    // println!("Parsing first finder.");
    let finder1 = parse_finder(components[1], &idx);
    // println!("Parsing second finder.");
    let finder2 = parse_finder(components[2], &idx);
    let command = parse_command(components[3], definitions, &idx);
    println!("File: {}, Finder1: {}, Finder2: {}, Command: {}", file, finder1, finder2, command);
    LineCommand::Insert(file.to_string(), finder1, finder2, command)
}

fn parse_command(insert_command: &str, definitions: &Vec<Definition>, idx: &usize) -> InsertCommand {
    let command = get_command_args(insert_command, false);
    match command.command.to_lowercase().as_str() {
        "enumcasewithraw" => {
            if command.args.len() != 2 {
                panic!("Expected two arguments to command EnumCaseWithRaw on line {idx}");
            }
            let arg1 = parse_command_arg(&command.args[0], definitions);
            let arg2 = parse_command_arg(&command.args[1], definitions);
            InsertCommand::EnumCaseWithRaw(arg1, arg2)
        }
        _ => panic!("Command not found: {} on line {}", command.command, idx)
    }
}

fn parse_command_arg(arg: &str, definitions: &Vec<Definition>) -> String {
    if arg.contains("(") || arg.contains(")") {
        let command = get_command_args(arg, false);
        match command.command.to_lowercase().as_str() {
            "camelcase" => {
                let arg = command.args.first().expect("Invalid set of arguments to camel case command");
                let value = parse_command_arg(arg, definitions);
                let recased = &value.to_case(convert_case::Case::Camel);
                recased.to_string()
            }
            _ => panic!("Invalid command {}.", command.command)
        }
    } else if arg.contains("$") {
        let stripped_arg = arg.strip_prefix("$").expect("Error stripping dynamic argument.");
        for def in definitions {
            if def.key == stripped_arg {
                return def.value.to_string();
            }
        }
        panic!("Program Error: No definition found for {stripped_arg}.");
    } else {
        arg.to_string()
    }
}

fn parse_finder(finder: &str, idx: &usize) -> Finder {
    let command = get_command_args(finder, false);
    if command.command.contains("lineregex") {
        if command.args.len() < 2 {
            panic!("Not enough arguments to finder command {} on line {idx}", command.command);
        }
        let lines = command.args[1].parse::<i32>().expect("Expected integer argument to LineRegex on line {idx}");
        return Finder::LineRegex(command.args.first().unwrap().clone().strip_prefix("\"").expect("Unknown error").strip_suffix("\"").expect("Unknown error").to_string(), lines);
    }  else if command.command.contains("regex") {
        if command.args.len() < 2 {
            panic!("Not enough arguments to finder command {} on line {idx}", command.command);
        }
        let lines = command.args[1].parse::<bool>().expect("Expected boolean argument to Regex on line {idx}");
        return Finder::Regex(command.args.first().unwrap().clone().strip_prefix("\"").expect("Unknown error").strip_suffix("\"").expect("Unknown error").to_string(), lines);

    } else if command.command.contains("lastinsectionorfallback") {
        if command.args.len() < 2 {
            panic!("Not enough arguments to finder command {} on line {idx}", command.command);
        }
        // println!("Recursing on finder {}", &command.args[0]);
        let nested_finder = parse_finder(&command.args[0], idx);
        // println!("Recursing on finder {}", &command.args[1]);
        let nested_fallback_finder = parse_finder(&command.args[1], idx);
        let mut options: Vec<FinderOption> = Vec::new();
        for item in &command.args[2..command.args.len()] {
            options.push(parse_finder_option(item, idx));
        }
        return Finder::LastInSectionOrFallback(Box::new(nested_finder), Box::new(nested_fallback_finder), options);
    }
    panic!("Invalid finder command on line {idx}: {}", command.command);
}

fn parse_finder_option(option: &str, idx: &usize) -> FinderOption {
    match option.to_lowercase().as_str() {
        "iffallbacknewlinebefore" => FinderOption::IfFallbackExtraNewline,
        _ => panic!("Invalid finder option {option} on line {idx}")
    }
}



// MARK: Command extraction
struct Command {
    command: String,
    args: Vec<String>
}

fn get_command_args(str: &str, print_output: bool) -> Command {
    // println!("Getting arguments for {str}");
    if str.contains("(") || str.contains(")") {
        let command = string_utils::remove_paren_contents(str).to_lowercase().trim().to_string();
        let arguments = string_utils::paren_contents(str);
        if print_output {
            println!("Command: {command}, Arguments: {arguments}");
        }
        let output = separate_by_toplevel_commas(&arguments);
        Command {
            command,
            args: output
        }
    } else {
        Command {
            command: str.to_string(),
            args: [].to_vec()
        }
    }
}

fn separate_by_toplevel_commas(str: &str) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut accumulator: String = "".to_string();
    let chars = str.chars();
    let mut open_paren_count: i16 = 0;
    for char in chars {
        match char {
            ',' => {
                if open_paren_count == 0
                    && !accumulator.is_empty() {
                        output.push(accumulator.trim().to_string());
                        accumulator = "".to_string();
                    } else if open_paren_count > 0 {
                    accumulator.push(char)
                }
            }
            '(' => {
                open_paren_count += 1;
                accumulator.push(char)
            }
            ')' => {
                open_paren_count -= 1;
                accumulator.push(char)

            }
            _ => accumulator.push(char)
        }
    }
    if !accumulator.is_empty() {
        output.push(accumulator.trim().to_string());
    }
    output
}
