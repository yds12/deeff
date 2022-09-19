use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use crate::CFG;

#[derive(Debug, PartialEq)]
pub struct Offset(String);

#[derive(Debug, PartialEq)]
pub struct Label(String, String, Offset);

impl Label {
    pub fn demangled_name(&self) -> &str {
        &self.0
    }

    pub fn name(&self) -> &str {
        &self.1
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction(String, Offset, String, String);

impl Instruction {
    pub fn op(&self) -> &str {
        &self.0
    }

    pub fn content(&self) -> &str {
        &self.2
    }
}

#[derive(Debug, PartialEq)]
pub struct SectionHeader(String);

impl SectionHeader {
    pub fn name(&self) -> &str {
        &self.0
    }
}

static RE_HASH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"::h[a-f0-9]{16}").expect("bug: wrong regex"));

fn demangle(id: &str) -> String {
    let st = rustc_demangle::demangle(id).to_string();

    if CFG.remove_hashes {
        RE_HASH
            .replace(&st, |caps: &regex::Captures| "")
            .to_string()
    } else {
        st
    }
}

#[derive(Debug, PartialEq)]
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
    Regex::new(" [ ]*([0-9a-f][0-9a-f]*):\t([a-z][a-z0-9]*)(.*)").expect("bug: wrong regex")
});

// taken from the `rustfilt` crate
static RE_SYM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"_(ZN|R)[\$\._[:alnum:]]*").expect("bug: wrong regex"));

impl Line {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Label(l) => &l.0,
            Self::Instruction(i) => &i.2,
            Self::SectionHeader(h) => &h.0,
            Self::Blank => "",
            Self::Other(o) => &o,
        }
    }

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
            let content = match RE_INSTR.captures(string).unwrap().get(3) {
                Some(val) => val.as_str(),
                _ => "",
            };
            Self::instruction(name, offset, content, string)
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
        let demangled_name: Cow<str> =
            RE_SYM.replace(name, |caps: &regex::Captures| demangle(&caps[0]));
        Self::Label(Label(
            demangled_name.into_owned(),
            name.to_owned(),
            Offset(offset.to_owned()),
        ))
    }

    fn instruction(name: &str, offset: &str, content: &str, line: &str) -> Self {
        //let line: Cow<str> = RE_SYM.replace(line, |caps: &regex::Captures| demangle(&caps[0]));
        let content: Cow<str> =
            RE_SYM.replace(content, |caps: &regex::Captures| demangle(&caps[0]));
        Self::Instruction(Instruction(
            name.to_owned(),
            Offset(offset.to_owned()),
            format!("{}{}", name, content),
            line.to_owned(),
        ))
    }
}
