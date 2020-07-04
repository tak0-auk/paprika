
pub fn start_with_prefix(s: &str) -> bool {
    let is_short = match s.chars().next() {
        Some(c) => c == '-',
        None => false
    };
    if !is_short {
        return false
    }
    let is_long = match s.chars().next() {
        Some(c) => c == '-',
        None => return false
    };
    is_short || is_long
}

pub fn trim_prefix(s: &str) -> &str {
    let mut pos = 0;
    for (i, c) in s.char_indices() {
        if c != '-' {
            pos = i;
            break;
        }
    }
    &s[pos..]
}


// テキトー過ぎるので後で直す
pub fn sprit_value(arg: &str) -> (&str, Option<&str>) {
    match arg.find('=') {
        Some(p) => {
            let (f, s) = arg.split_at(p);
            (f, Some(&s[1..]))
        },
        None => (arg, None)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_start_with_prefix() {
        assert!(start_with_prefix("-h"));
        assert!(!start_with_prefix("h"));
    }

    #[test]
    fn test_trim_prefix() {
        assert_eq!("v", trim_prefix("-v"));
        assert_eq!("version", trim_prefix("--version"));
        assert_eq!("test", trim_prefix("test"));
    }

    #[test]
    fn test_split_value() {
        assert_eq!(("A", Some("text")), sprit_value("A=text"));
        // assert_eq!(("Xms", Some("8192")), sprit_value("Xms8192"));
        assert_eq!(("c", None), sprit_value("c"));
    }
}