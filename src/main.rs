extern crate paprika;

use paprika::{App, Ops};

fn main() {

    let mut app = App::new();
    let ver = Ops::new()
                .short("v")
                .long("version")
                .description("version")
                .only(true);
    let help = Ops::new()
                .short("h")
                .description("Prints help information")
                .only(true);
    let xms = Ops::new()
                .short("Xms")
                .takes_value(true);
    app.add_ops(ver);
    app.add_ops(help);
    app.add_ops(xms);

    app.parse();

}