use once_cell::sync::Lazy;
use regex::Regex;

static RE_HASH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"::h[a-f0-9]{16}").expect("bug: wrong regex"));

pub fn demangle(id: &str) -> String {
    dbg!(id);
    let val = rustc_demangle::demangle(id).to_string();
    dbg!(&val);
    val
}

pub fn demangle_no_hash(id: &str) -> String {
    let st = demangle(id);
    RE_HASH.replace(&st, |_: &regex::Captures| "").to_string()
}
