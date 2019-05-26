use std::env;

use crate::parser::parse_arg;

pub struct App {
    args: Vec<String>,
    ops: Vec<Ops>,
    about: String,
    prefix: String,
    conjunction: String,
}

pub struct Ops {
    short: String,
    long: String,
    description: String,
    takes_value: bool,
    is_exist: bool,
    value: String,
    is_only: bool,
}

impl App {

    pub fn new() -> App {
        App{
            args: env::args().collect(),
            ops: vec![],
            about: String::default(),
            prefix: String::from("-"),
            conjunction: String::from("=")
        }
    }

    pub fn exec(&mut self) {
        self.parse();
    }

    pub fn add_ops(&mut self, ops: Ops) {
        self.ops.push(ops);
    }

    pub fn has_ops(&self, s: &str) -> bool {
        match self.ops.iter().find(|o| o.is_exist && o.short == s) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_value(&self, ops: &str) -> String {
        panic!("unimplemented")
    }

    pub fn print(&self) {
        println!("{}", self.about);
        for o in &self.ops {
            println!("\t-{} --{}\t\t{}", o.short, o.long, o.description);
        }
    }
}

impl App {

    fn parse(&mut self) {
        for arg in &self.args {
            if arg.starts_with(&self.prefix) {
                for o in &mut self.ops {
                    if o.short == arg.trim_start_matches(&self.prefix) {
                        o.is_exist = true;
                    }
                }
            }
        }
    }
}

impl Ops {
    pub fn new() -> Ops {
        Ops{
            short: String::default(),
            long: String::default(),
            description: String::default(),
            takes_value: false,
            is_exist: false,
            value: String::default(),
            is_only: false
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

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn takes_value(mut self, is_takes: bool) -> Self {
        self.takes_value = is_takes;
        self
    }

    pub fn only(mut self, only: bool) -> Self {
        self.is_only = only;
        self
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn has_short_flag() {
        let mut app = App::new();
        app.args = vec!["-v".to_string()];
        let ver = Ops::new()
                .short("v");
        app.add_ops(ver);
        app.exec();
        assert!(app.has_ops("v"));
    }

    #[test]
    #[should_panic]
    fn get_ops_value() {
        unimplemented!();
    }

}