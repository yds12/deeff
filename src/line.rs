use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Offset(String);

#[derive(Debug)]
pub struct Label(String, Offset);

#[derive(Debug)]
pub struct Instruction(String, Offset, String);

#[derive(Debug)]
pub struct SectionHeader(String);

fn demangle(id: &str) -> String {
    rustc_demangle::demangle(id).to_string()
}

#[derive(Debug)]
pub enum Line {
    Label(Label),
    Instruction(Instruction),
    SectionHeader(SectionHeader),
    Blank,
    Other(String),
}

static RE_HEADER: Lazy<Regex> =
    Lazy::new(|| Regex::new("Disassembly of section (.*):").expect("bug: wrong regex"));

static RE_LABEL: Lazy<Regex> =
    Lazy::new(|| Regex::new("([0-9a-f][0-9a-f]*) <(.*)>:").expect("bug: wrong regex"));

static RE_INSTR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(" [ ]*([0-9a-f][0-9a-f]*):\t([a-z][a-z0-9]*)[^a-z0-9]*").expect("bug: wrong regex")
});

// taken from the `rustfilt` crate
static RE_SYM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"_(ZN|R)[\$\._[:alnum:]]*").expect("bug: wrong regex"));

impl Line {
    pub fn from_str(string: &str) -> Self {
        if RE_HEADER.is_match(string) {
            let cap = RE_HEADER.captures(string).unwrap().get(1).unwrap().as_str();
            Self::section_header(cap)
        } else if RE_LABEL.is_match(string) {
            let offset = RE_LABEL.captures(string).unwrap().get(1).unwrap().as_str();
            let name = RE_LABEL.captures(string).unwrap().get(2).unwrap().as_str();
            Self::label(name, offset)
        } else if RE_INSTR.is_match(string) {
            let offset = RE_INSTR.captures(string).unwrap().get(1).unwrap().as_str();
            let name = RE_INSTR.captures(string).unwrap().get(2).unwrap().as_str();
            Self::instruction(name, offset, string)
        } else if string.trim().is_empty() {
            Self::Blank
        } else {
            Self::Other(string.to_owned())
        }
    }

    fn section_header(name: &str) -> Self {
        Self::SectionHeader(SectionHeader(name.to_owned()))
    }

    fn label(name: &str, offset: &str) -> Self {
        let name: Cow<str> = RE_SYM.replace(name, |caps: &regex::Captures| demangle(&caps[0]));
        Self::Label(Label(name.into_owned(), Offset(offset.to_owned())))
    }

    fn instruction(name: &str, offset: &str, line: &str) -> Self {
        let line: Cow<str> = RE_SYM.replace(line, |caps: &regex::Captures| demangle(&caps[0]));
        Self::Instruction(Instruction(
            name.to_owned(),
            Offset(offset.to_owned()),
            line.into_owned(),
        ))
    }
}
