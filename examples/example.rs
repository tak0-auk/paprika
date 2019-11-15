extern crate paprika;

use paprika::{App, Ops};

use std::process;

fn main() {

    let mut app = App::new();
    let ver = Ops::new()
                .short("v")
                .long("version")
                .description("version");
    app.add_ops(ver);

    app.parse();
    if app.has_ops("version") {
        println!("version {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

}