use std::io::{Error, ErrorKind};
use std::env;

pub fn handling_arguments() -> Result<String, Error>{
    if env::args().len() < 2 {
        return Err(Error::new(ErrorKind::Other,"too little arguments"));
    }

    let argv: Vec<String> = env::args().collect();

    for index in 0..argv.len() {
        if argv[index] == "-file" {
            return Ok(argv.into_iter().nth(index + 1).unwrap());
        }
    }
    
    Err(Error::new(ErrorKind::Other,"Unkown command"))
}

