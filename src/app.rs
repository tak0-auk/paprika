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
    description: String,
    takes_value: bool,
}

#[derive(Default)]
pub struct Arg {
    ops_short: String,
    ops_long: String,
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

    pub fn parse(&mut self) {
        let args: Vec<String> = env::args().collect();
        self.app_path = args[0].clone();
        self.args = self.matcher(args[1..].to_vec());


        //     println!("ops exists");
        //     let flag_value: Vec<&str> = arg.split(&self.conjunction).collect();
        //     let name = flag_value[0];
        //     let mut value: String = String::default();
        //     if ops.takes_value {
        //         value = flag_value[1].to_string();
        //     }
        //     a.push(

        //     );
        // }

    }

    pub fn add_ops(&mut self, ops: Ops) {
        self.ops.push(ops);
    }

    pub fn has_ops(&self, s: &str) -> bool {
        self.args.iter().any(|arg| (arg.ops_short == s || arg.ops_long == s))
    }

    pub fn get_value(&self, ops: &str) -> Option<String> {
        match self.get_arg(ops.to_string()) {
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
            println!("\t\t{}", o.description);
        }
    }
}

impl App {

    fn matcher(&self, e: Vec<String>) -> Vec<Arg> {

        let mut args: Vec<Arg> = vec![];
        for (_i, arg) in e.iter().enumerate() {
            let (name, value) = self.separate_args(arg.clone());
            let ops = match self.find_ops(name) {
                Some(o) => o,
                None => continue
            };
            args.push(Arg {
                    ops_short: ops.short.clone(),
                    ops_long: ops.long.clone(),
                    value: value
            });
        }
        return args;
    }

    fn separate_args(&self, s: String) -> (String, Option<String>) {
        let v: Vec<&str> = s.split(&self.conjunction).collect();
        let name: String;
        let raw = v[0];
        if raw.starts_with("--") {
            name = raw[2..].to_string();
        } else if raw.starts_with("-") {
            name = raw[1..].to_string();
        } else {
            name = raw.to_string();
        }
        let mut value: Option<String> = None;
        if v.len() > 1 {
            value = Some(v[1].to_string());
        }
        return (name, value)
    }

    fn find_ops(&self, s: String) -> Option<&Ops> {
        self.ops.iter().find(|o| o.short == s || o.long == s)
    }

    fn get_arg(&self, s: String) -> Option<&Arg> {
        self.args.iter().find(|a| a.ops_short == s || a.ops_long == s)
    }
}

impl Ops {
    pub fn new() -> Ops {
        Ops{
            short: String::default(),
            long: String::default(),
            description: String::default(),
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

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
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
                        ops_short: "Xms".to_string(),
                        ops_long: String::default(),
                        value: Some("4096".to_string()),
            },
            Arg {
            ops_short: "v".to_string(),
            ops_long: "version".to_string(),
            value: None,
            },
        ];
        app
    }

    #[test]
    fn has_flag() {
        let mut app = setup();
        let ver = Ops::new()
                .short("v")
                .long("version");
        app.add_ops(ver);
        assert!(app.has_ops("v"));
        assert!(app.has_ops("version"));
    }

    #[test]
    fn has_ops(){
        let app = setup();
        assert!(app.has_ops("v"));
    }

    #[test]
    fn not_has_ops() {
        let mut app = setup();
        app.parse();
        assert!(!app.has_ops("v"));
    }

    #[test]
    fn get_value() {
        let app = setup();
        assert_eq!(app.get_value("Xms"), Some("4096".to_string()));
    }

    // #[test]
    // #[should_panic]
    // fn app_panic() {
    //     unimplemented!();
    // }

}