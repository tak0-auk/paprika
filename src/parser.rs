extern crate regex;

pub fn parse_arg(prefix: &str, a: String) {
    a.starts_with(prefix);
    a.trim_start_matches(prefix);
}