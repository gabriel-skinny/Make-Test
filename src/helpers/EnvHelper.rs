use std::io::{Error, ErrorKind};
use std::env;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Arguments {
   pub file_name: String,
   pub filter_path: String,
} 


pub fn get_arguments() -> Result<Arguments, Error>{
    if env::args().len() < 2 {
        return Err(Error::new(ErrorKind::Other,"too little arguments"));
    }

    let mut arguments_hash_map: HashMap<String, String> = make_hash_map();

    let argv: Vec<String> = env::args().collect();
    let mut argument_key = 1;
    let mut argument_value = 2;
    let mut iteration = 0;
    while argument_value <= argv.len() - 1 || argument_key < argv.len() - 1 {
        handle_argument(&argv[argument_key], &argv[argument_value], &mut arguments_hash_map)?;
        argument_key += 2;
        argument_value = argument_key + 1;
        iteration += 1;
    }

    let arguments = transform_hash_in_struct(arguments_hash_map);

    if iteration == 0 {
        return Err(Error::new(ErrorKind::Other, "Unkown command"));
    }

    Ok(arguments)
}

fn make_hash_map() -> HashMap<String, String> {
    HashMap::from([
                  ("--file".to_owned(), String::new()),
                  ("--filter-path".to_owned(), String::from("src/")),
    ])
}

fn handle_argument(arg: &str, value: &str, arguments: &mut HashMap<String, String>) -> Result<(), Error> {
    match arguments.get_mut(arg) {
        Some(key) => {
            *key = value.to_owned();
        }
        None => {
            return Err(Error::new(ErrorKind::Other,"Unkown command"));
        }
    }

    Ok(())
}

fn transform_hash_in_struct(hash: HashMap<String, String>) -> Arguments {
    Arguments {
        file_name: hash.get("--file").unwrap().to_owned(),
        filter_path: hash.get("--filter-path").unwrap().to_owned()
    }
}


