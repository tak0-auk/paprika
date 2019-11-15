# PAPRIKA 

[![Build Status](https://travis-ci.org/tak0-auk/paprika.svg?branch=master)](https://travis-ci.org/tak0-auk/paprika)

PAPRIKA is Command Line Parser like a toy.  
PAPRIKA is a product for learning Rust and version control.  
If you need a good command line parser,
You should use Clap instead of PAPRIKA.


```
[dependencies]
paprika = "*"
```

```Rust
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
```
```
cargo run --example example -- -v
```

```
cargo run --example get_value --  --name="YOUR NAME"
```

Well, this is just killing time.

By the way, I am not good at English. I would be very happy to tell if there are any strange grammars.