extern crate paprika;
use paprika::{App, Ops};

use std::process;

fn main() {

    let mut app = App::new();
    let name = Ops::new()
                .long("name")
                .takes_value(true);
    app.add_ops(name);

    app.parse();
    if app.has_ops("name") {
        match app.get_value("name"){
            Some(val) => println!("Hello {}!!", val),
            None => panic!()
        }
    }

}