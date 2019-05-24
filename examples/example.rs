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
    app.add_ops(ver);
    app.add_ops(help);
    
    app.exec();

}