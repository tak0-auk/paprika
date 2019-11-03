extern crate paprika;
use paprika::{App, Ops};

use std::process;

fn main() {

    let mut app = App::new();
    let ver = Ops::new()
                .short("v")
                .long("version")
                .description("version");
    let help = Ops::new()
                .short("h")
                .description("Prints help information");
    let xms = Ops::new()
                .long("Xms")
                .takes_value(true);
    app.add_ops(ver);
    app.add_ops(help);
    app.add_ops(xms);

    app.parse();
    if app.has_ops("version") {
        println!("version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }
    if app.has_ops("Xms") {
        match app.get_value("Xms"){
            Some(val) => println!("{}", val),
            None => panic!()
        }
    }

}