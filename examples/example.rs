extern crate paprika;

use paprika::{App, Ops};

use std::process;

fn main() {

    let mut app = App::new();
    let ver = Ops::new()
                .short("v")
                .long("version");
    app.add_ops(ver);

    app.parse();
    app.print();
    if app.has_ops("version") {
        println!("version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    if app.take_args() {
        println!("hello");
    }

}