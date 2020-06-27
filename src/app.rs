use std::env;
use crate::parser::*;

#[derive(Default)]
pub struct App {
    app_path: String,
    args: Vec<Arg>,
    ops: Vec<Ops>,
    about: String,
}

#[derive(Default)]
pub struct Ops {
    short: String,
    long: String,
    takes_value: bool,
    conjunction: Option<char>,
}

#[derive(Default)]
pub struct Arg {
    raw:    String,
    pos:    usize,
    ops:    Option<Ops>,
    value:  Option<String>,
}

impl App {

    pub fn new() -> App {
        App{
            app_path: String::default(),
            args: vec![],
            ops: vec![],
            about: String::default()
        }
    }


    pub fn add_ops(&mut self, ops: Ops) {
        self.ops.push(ops);
    }

    pub fn parse(&mut self) {
        let arguments: Vec<String> = env::args().collect();
        self.app_path = arguments[0].clone();
        self.args = self.matcher(arguments[1..].to_vec());
    }

    pub fn has_ops(&self, s: &str) -> bool {
        self.args.iter().any(|arg| 
            match &arg.ops {
                Some(o) => o.short == s || o.long == s,
                None => false,
            }
        )
    }

    pub fn take_args(&self) -> bool {
        self.args.len() >= 1
    }

    pub fn get_value(&self, ops: &str) -> Option<String> {
        match self.find_args(ops.to_string()) {
            Some(a) => a.value.clone(),
            None => None,
        }
    }

    pub fn print(&self) {
        println!("{}", self.about);
        for o in &self.ops {
            print!("\t");
            if !o.short.is_empty() {
                print!("-{} ", o.short);
            }
            if !o.long.is_empty() {
                print!("--{}", o.long);
            }
        }
    }
}

impl App {

    fn matcher(&mut self, arguments: Vec<String>) -> Vec<Arg> {
        let mut args: Vec<Arg> = vec![];
        for (i, arg) in arguments.iter().enumerate() {

            if arg == "-" || arg == "--" {
                args.push(
                    Arg {
                        raw: arg.to_string(),
                        pos: i,
                        ops: None,
                        value: None,
                    }
                );
                continue;
            }

            let foo = trim_prefix(arg).to_string();

            let n = if let Some(p) = self.ops.iter().position(|o| o.is_myself(&foo)) {
                Arg {
                    raw: arg.to_string(),
                    pos: i,
                    ops: Some(self.ops.remove(p)),
                    value: self.extract_value(foo),
                }
            } else {
                Arg {
                    raw: arg.to_string(),
                    pos: i,
                    ops: None,
                    value: None
                }
            };

            args.push(n);

        }

        args
    }

    fn extract_value(&self, arg: String) -> Option<String> {
        let (_, val) = sprit_value(&arg);
        match val {
            Some(v) => Some(v.to_string()),
            None => None
        }
        
    }

    fn find_args(&self, s: String) -> Option<&Arg> {
        self.args.iter().find(|&arg| match &arg.ops {
            Some(o) => o.short == s || o.long == s,
            None => false
        })
    }
}

impl Ops {
    pub fn new() -> Ops {
        Ops{
            short: String::default(),
            long: String::default(),
            takes_value: false,
            conjunction: Some('='),
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

    fn is_myself(&self, s: &str) -> bool {
        true
    }

}


#[cfg(test)]
mod test {

    use super::*;

    fn setup() -> App {
        let mut app = App::new();
        app.matcher(vec![
            "-v".to_string(),
            "-Xms4096".to_string(),
            "--FILE=example.txt".to_string(),
            "-".to_string(),
            "LAST!".to_string()]
        );

        let version = Ops::new()
                .short("v")
                .long("version");
        let xms = Ops::new()
                .short("Xms")
                .long("version");
        let file = Ops::new()
                .long("FILE")
                .takes_value(true);
        
        app.add_ops(version);
        app.add_ops(xms);
        app.add_ops(file);
        app
    }

    #[test]
    fn has_ops() {
        let mut app = setup();
        app.parse();
        assert_eq!(true, app.has_ops("version"));
    }


    #[test]
    fn get_value() {
        let mut app = setup();
        app.parse();
        // assert_eq!(app.get_value("Xms"), Some("4096".to_string()));
        assert_eq!(app.get_value("FILE"), Some("example.txt".to_string()));
        assert_eq!(app.get_value("version"), None);
    }

    #[test]
    fn take_args() {
        let mut app = App::new();
        app.parse();
        assert!(!app.take_args());
    }

    // fn next_args() {
    //     let app = setup();
    //     app.parse();
    //     let arg = app.next_arg("");
    // }

}