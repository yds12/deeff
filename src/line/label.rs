use std::borrow::Cow;
use super::{demangle, demangle_no_hash, Offset, RE_LABEL, RE_SYM};

#[derive(Debug, PartialEq)]
pub struct Label {
    /// Original label name
    name: String,
    /// Name after rust demangling
    demangled_name: String,
    /// Name after demangling and removing hash
    clean_name: String,
    offset: Offset,
}

impl Label {
    pub fn new(line: &str) -> Self {
        let offset = RE_LABEL.captures(line).unwrap().get(1).unwrap().as_str();
        let name = RE_LABEL.captures(line).unwrap().get(2).unwrap().as_str();

        let demangled_name: Cow<str> =
            RE_SYM.replace(name, |caps: &regex::Captures| demangle(&caps[0]));
        let clean_name: Cow<str> =
            RE_SYM.replace(name, |caps: &regex::Captures| demangle_no_hash(&caps[0]));
        Self {
            name: name.to_owned(),
            demangled_name: demangled_name.into_owned(),
            clean_name: clean_name.into_owned(),
            offset: Offset(offset.to_owned()),
        }
    }

    pub fn clean_name(&self) -> &str {
        &self.clean_name
    }

    pub fn demangled_name(&self) -> &str {
        &self.demangled_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
