use once_cell::sync::Lazy;
use regex::Regex;

mod instruction;
mod label;
mod mangle;

use instruction::Instruction;
use label::Label;
use mangle::{demangle, demangle_no_hash};

#[derive(Debug, Clone, PartialEq)]
pub struct Offset(String);

#[derive(Debug, Clone, PartialEq)]
pub struct SectionHeader(String);

impl SectionHeader {
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsmLine {
    Label(Label),
    Instruction(Instruction),
    SectionHeader(SectionHeader),
    Blank,
    Other,
}

impl AsmLine {
    fn section_header(name: &str) -> Self {
        Self::SectionHeader(SectionHeader(name.to_owned()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    raw: String,
    inner: AsmLine,
}

impl std::default::Default for Line {
    fn default() -> Self {
        Self {
            raw: String::new(),
            inner: AsmLine::Blank,
        }
    }
}

static RE_HEADER: Lazy<Regex> =
    Lazy::new(|| Regex::new("Disassembly of section (.*):").expect("bug: wrong regex"));

static RE_LABEL: Lazy<Regex> =
    Lazy::new(|| Regex::new("([0-9a-f][0-9a-f]*) <(.*)>:").expect("bug: wrong regex"));

// taken from the `rustfilt` crate
static RE_SYM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"_(ZN|R)[\$\._[:alnum:]]*").expect("bug: wrong regex"));

static RE_INSTR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(" [ ]*([0-9a-f][0-9a-f]*):\t([a-z][a-z0-9]*)[ ]*([^,]*)(?:,([^,]*))*")
        .expect("bug: wrong regex")
});

impl Line {
    pub fn into_inner(self) -> AsmLine {
        self.inner
    }

    pub fn inner(&self) -> &AsmLine {
        &self.inner
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }

    pub fn from_str(line: &str) -> Self {
        let inner = if RE_HEADER.is_match(line) {
            let cap = RE_HEADER.captures(line).unwrap().get(1).unwrap().as_str();
            AsmLine::section_header(cap)
        } else if RE_LABEL.is_match(line) {
            AsmLine::Label(Label::new(line))
        } else if RE_INSTR.is_match(line) {
            AsmLine::Instruction(Instruction::new(line))
        } else if line.trim().is_empty() {
            AsmLine::Blank
        } else {
            AsmLine::Other
        };

        Self {
            raw: line.to_owned(),
            inner,
        }
    }
}
