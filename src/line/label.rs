use super::{demangle, demangle_no_hash, Offset, RE_LABEL, RE_SYM};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demangled() {
        let line = "00000000000462c0 <_ZN4core7unicode12unicode_data2cc6lookup17h4f90392d718973aaE>:";
        assert_eq!(Label::new(line).demangled_name(), "core::unicode::unicode_data::cc::lookup::h4f90392d718973aa");
    }

    #[test]
    fn demangled_without_hash() {
        let line = "00000000000462c0 <_ZN4core7unicode12unicode_data2cc6lookup17h4f90392d718973aaE>:";
        assert_eq!(Label::new(line).clean_name(), "core::unicode::unicode_data::cc::lookup");
    }
}
