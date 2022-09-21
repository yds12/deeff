use regex::Regex;
use once_cell::sync::Lazy;

static RE_HASH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"::h[a-f0-9]{16}").expect("bug: wrong regex"));

pub fn demangle(id: &str) -> String {
    rustc_demangle::demangle(id).to_string()
}

pub fn demangle_no_hash(id: &str) -> String {
    let st = demangle(id);
    RE_HASH.replace(&st, |_: &regex::Captures| "").to_string()
}
