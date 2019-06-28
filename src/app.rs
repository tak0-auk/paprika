use std::env;

#[derive(Default)]
pub struct App {
    app_path:String,
    args: Vec<String>,
    ops: Vec<Ops>,
    about: String,
    prefix: String,
    conjunction: String,
}

#[derive(Default)]
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
        let a:Vec<String> = env::args().collect();
        App{
            app_path: a[0].clone(),
            args: env::args().skip(1).collect(),
            ops: vec![],
            about: String::default(),
            prefix: String::from("-"),
            conjunction: String::from("=")
        }
    }

    pub fn parse(&mut self) {
        let lp = format!("{}{}", self.prefix, self.prefix);
        for arg in &self.args {
            let sp: Vec<&str> = arg.split(&self.conjunction).collect();
            let name = sp[0];
            for o in self.ops.iter_mut().filter(|p| !p.is_exist) {
                if arg.starts_with(&lp) {
                    if o.long == name[2..] {
                        o.is_exist = true;
                        if o.takes_value {
                            o.value = sp[1].to_string();
                        }
                    }
                    
                }else if arg.starts_with(&self.prefix) {
                    if o.short == name[1..] {
                        o.is_exist = true;
                        if o.takes_value {
                            o.value = sp[1].to_string();
                        }
                    }
                }else {
                    println!("{}", arg);
                }
            }
        }
    }

    pub fn add_ops(&mut self, ops: Ops) {
        self.ops.push(ops);
    }

    pub fn has_ops(&self, s: &str) -> bool {
        self.ops.iter().any(|o| o.is_exist && (o.short == s || o.long == s))
    }

    pub fn get_value(&self, ops: &str) -> String {
        self.get_ops(ops).value.clone()
    }

    pub fn print(&self) {
        println!("{}", self.about);
        for o in &self.ops {
            print!("\t");
            if !o.short.is_empty() {
                print!("-{}  ", o.short);
            }
            if !o.long.is_empty() {
                print!("--{}", o.long);
            }
            println!("\t\t{}", o.description);
        }
    }
}

impl App {

    fn get_ops(&self, name: &str) -> &Ops {
        match self.ops.iter().find(|o| o.is_exist && (o.short == name || o.long == name)) {
            Some(o) => o,
            None => panic!("Option not found"),
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
        app.parse();
        assert!(app.has_ops("v"));
    }

    #[test]
    fn has_long_flag() {
        let mut app = App::new();
        app.args = vec!["--help".to_string()];
        let ver = Ops::new()
                .long("help");
        app.add_ops(ver);
        app.parse();
        assert!(app.has_ops("help"));
    }

    #[test]
    fn get_ops_value() {
        let mut app = App::new();
        app.args = vec!["-Xms=4096".to_string()];
        let ver = Ops::new()
                    .short("Xms")
                    .takes_value(true);
        app.add_ops(ver);
        app.parse();
        assert_eq!(app.get_value("Xms"), "4096");
    }

    #[test]
    #[should_panic]
    fn app_panic() {
        unimplemented!();
    }

}