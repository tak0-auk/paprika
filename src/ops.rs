#[derive(Default)]
pub struct Ops {
    short: String,
    long: String,
    takes_value: bool,
    // conjunction: Option<char>,
}

impl Ops {
    pub fn new() -> Ops {
        Ops{
            short: String::default(),
            long: String::default(),
            takes_value: false,
            // conjunction: Some('='),
        }
    }

    pub fn short(mut self, s: &str) -> Self {
        self.short = s.to_string();
        self
    }

    pub fn long(mut self, l: &str) -> Self {
        self.long = l.to_string();
        self
    }

    pub fn takes_value(mut self, is_takes: bool) -> Self {
        self.takes_value = is_takes;
        self
    }

    pub fn is_myself(&self, s: &str) -> bool {
        let len = s.len();
        let is_short = if self.short.len() <= len {
            self.short == s[0..self.short.len()]
        } else {
            false
        };

        let is_long = if self.long.len() <= len {
            self.long == s[0..self.long.len()]
        } else {
            false
        };
        is_short || is_long
    }

}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_is_myself() {
        let ops = Ops::new().short("h");
        assert!(ops.is_myself("h"));
    }
}