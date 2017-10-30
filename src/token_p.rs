use std::env;
use std::process;
use std::io::{self, Write};

struct Token {
    kind:               Kind,
    text[TEXT_SIZE+1]:  char,
    intVal:             int,
}

enum Kind {
    
}

fn main() {
    let args: Vec<_>  = env::args.collect();
    
    process::exit(match run_app() {
        Ok(_) => 0,
        Error(err) => {
            writeln!(io::stderr(),"error: {:?}", err).unwrap();
            1
        }
    });
}

fn run_app() -> Result<(), ()> {
    initChTyp();

    Ok(())
}
