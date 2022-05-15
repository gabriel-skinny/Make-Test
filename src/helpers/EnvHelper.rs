use std::io::{Error, ErrorKind};
use std::env;

#[derive(Default, Debug)]
pub struct Arguments {
   pub file_name: String,
   pub filter_path: String,
} 


pub fn get_arguments() -> Result<Arguments, Error>{
    if env::args().len() < 2 {
        return Err(Error::new(ErrorKind::Other,"too little arguments"));
    }

    let mut arguments = Arguments::default();

    let argv: Vec<String> = env::args().collect();
    let mut argument_key = 1;
    for mut index in 1..argv.len() {
            let argument_value = index + 1;
            if argument_value  > argv.len() - 1 || argument_key > argv.len() - 1 { break };
            handle_argument(&argv[argument_key], &argv[argument_value], &mut arguments)?;
            argument_key += 2;
    }
    
    Ok(arguments)
}

fn handle_argument(arg: &str, next_argv: &str, arguments: &mut Arguments) -> Result<(), Error> {
    if arg == "--file" {
        arguments.file_name = next_argv.to_owned();
    }
    else if arg ==  "--filter-path" {
        arguments.filter_path = next_argv.to_owned();
    } else {
        return Err(Error::new(ErrorKind::Other,"Unkown command"));
    }

    Ok(())
}
