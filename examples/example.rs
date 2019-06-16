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
                .long("Xms")
                .takes_value(true);
    app.add_ops(ver);
    app.add_ops(help);
    app.add_ops(xms);

    app.parse();
    app.print();
    if app.has_ops("version") {
        println!("version {}", env!("CARGO_PKG_VERSION"));
    }
    if app.has_ops("Xms") {
        let x = app.get_value("Xms");
        println!("{}", x);
    }

}