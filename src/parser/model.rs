// regex("case unused");lastInSectionOrFallback($section, linesBeforeFind(3), ifFallbackNewlineBefore);enumCaseWithRaw(camelCase($key), $key)

use std::fmt::Display;

pub enum LineCommand {
    // Filename, where to start looking, where to make the insertion, what to insert.
    Insert(String, Finder, Finder, InsertCommand),
    Lang(SupportedLanguage),
    Def(Definition),
    None
}

pub struct Definition {
    pub key: String,
    pub value: String
}

pub enum SupportedLanguage {
    Swift
}

pub enum InsertCommand {
    // key, raw
    EnumCaseWithRaw(String, String)
}

impl Display for InsertCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertCommand::EnumCaseWithRaw(command, raw) => write!(f, "Command.EnumCaseWithRaw({command}, raw: {raw})"),
        }
    }
}

pub enum Finder {
    // Expression, then before/after = true/false
    Regex(String, bool),
    // Expression, then line offset from expression
    LineRegex(String, i32),
    LastInSectionOrFallback(Box<Finder>, Box<Finder>, Vec<FinderOption>),
}

impl Display for Finder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Finder::Regex(pattern, ba) => {
                let mut boolstr: &str = "after";
                if *ba { boolstr = "before" };
                write!(f, "Finder.Regex({pattern}, {boolstr})")
            }
            Finder::LastInSectionOrFallback(finder, finder1, vec) => {
                write!(f, "Finder.LastInSectionOrFallback(default: {finder}, fallback: {finder1}, options: {:?})", vec)
            },
            Finder::LineRegex(pattern, offset) => write!(f, "Finder.LineRegex({pattern}, offset: {offset})"),
        }
    }
}

#[derive(Debug, Copy, PartialEq)]
pub enum FinderOption {
    IfFallbackExtraNewline
}

impl Clone for FinderOption {
    fn clone(&self) -> Self {
        match self {
            FinderOption::IfFallbackExtraNewline => return FinderOption::IfFallbackExtraNewline,
        }
    }
}

impl Display for FinderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinderOption::IfFallbackExtraNewline => write!(f, "FinderOption.IfFallbackNewlineBefore"),
        }
    }
}

pub enum InsertionPoint {
    LastInSectionOrFallback
}

pub enum InsertionItem {
    EnumCaseWithRaw
}
