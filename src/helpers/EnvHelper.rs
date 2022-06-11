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

    let mut argv_iter = env::args().skip(1); 
    while let Some(arg) = argv_iter.next() {
       handle_argument(&arg, &argv_iter.next().unwrap(), &mut arguments_hash_map)?; 
    }

    let arguments = transform_hash_in_struct(arguments_hash_map);

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


