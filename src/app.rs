use std::env;

#[derive(Default)]
pub struct App {
    app_path: String,
    args: Vec<Arg>,
    ops: Vec<Ops>,
    about: String,
    conjunction: String,
}

#[derive(Default)]
pub struct Ops {
    short: String,
    long: String,
    takes_value: bool,
}

#[derive(Default)]
pub struct Arg {
    ops: Option<Ops>,
    value: Option<String>,
}

impl App {

    pub fn new() -> App {
        App{
            app_path: String::default(),
            args: vec![],
            ops: vec![],
            about: String::default(),
            conjunction: String::from("=")
        }
    }


///Add necessary options
    pub fn add_ops(&mut self, ops: Ops) {
        self.ops.push(ops);
    }

    pub fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        self.app_path = args[0].clone();
        self.args = self.matcher(args[1..].to_vec());
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

    fn matcher(&mut self, cl_args: Vec<String>) -> Vec<Arg> {
        let mut args: Vec<Arg> = vec![];
        let mut current_ops: Vec<Ops> = vec![];
        std::mem::swap(&mut self.ops, &mut current_ops);
        for (_i, arg) in cl_args.iter().enumerate() {
            let (is_ops, name, value) = self.separate_args(arg.clone());
            let _arg: Arg;
            if is_ops {
                if let Some(i) = current_ops.iter().position(|o| o.short == name || o.long == name) {
                    args.push(
                        Arg {
                            ops: Some(current_ops.remove(i)),
                            value
                        }
                    );
                };
            } else {
                args.push(Arg {
                    ops: None,
                    value: None
                });
            };
        }

        args
    }

    fn separate_args(&self, s: String) -> (bool, String, Option<String>) {
        let split_args: Vec<&str> = s.split(&self.conjunction).collect();
        let mut is_ops = false;
        let name: String;
        let front = split_args[0];
        if front.starts_with("-") {
            is_ops = true;
            if front.starts_with("--") {
                name = front[2..].to_string();
            } else {
                name = front[1..].to_string();
            }
            let value = if split_args.len() > 1 {
                Some(split_args[1].to_string())
            } else {
                None
            };
            (is_ops, name, value)
        } else {
            name = s.clone();
            (is_ops, name, None)
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
            takes_value: false
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

}


#[cfg(test)]
mod test {

    use super::*;

    fn setup() -> App {
        let mut app = App::new();
        app.args = vec![
             Arg {
                 ops: Some(Ops::new().short("v").long("--version")),
                 value: None
             },
             Arg {
                 ops: Some(Ops::new().short("Xms").takes_value(true)),
                 value: Some("4096".to_string())
             },
             Arg {
                 ops: None,
                 value: None
             }
        ];
        app
    }

    #[test]
    fn has_ops() {
        let app = setup();
        assert!(app.has_ops("v"));
    }


    #[test]
    fn get_value() {
        let app = setup();
        assert_eq!(app.get_value("Xms"), Some("4096".to_string()));
    }

    #[test]
    fn ver() {
        let mut app = App::new();
        app.parse();
       assert!(!app.take_args());
    }

}