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
    take_value: bool,
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

    pub fn exec(self) {
        for arg in &self.args {
            if arg.starts_with(&self.prefix) {
                for o in &self.ops {
                    if o.short == arg.trim_start_matches(&self.prefix) {
                        println!("has {}", arg);
                    }
                }
            }
        }
    }

    pub fn add_ops(&mut self, ops: Ops) {
        self.ops.push(ops);
    }

    pub fn print(&self) {
        println!("{}", self.about);
        for o in &self.ops {
            println!("\t-{} --{}\t\t{}", o.short, o.long, o.description);
        }
    }
}

impl App {

    fn parse(self) {
        let args: Vec<String> = env::args().collect();
        for arg in args {
            println!("{}", arg);
        }
    }
}

impl Ops {
    pub fn new() -> Ops {
        Ops{
            short: String::default(),
            long: String::default(),
            description: String::default(),
            take_value: false,
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

    pub fn only(mut self, only: bool) -> Self {
        self.is_only = only;
        self
    }
}
