use std::fs::File;
use std::io::{BufReader, Read, Error, ErrorKind};
use std::env;

fn read_file(file_name: &str) -> Result<String, Error>{
    let read_file = File::open(file_name)?;

    let mut buff_read = BufReader::new(read_file);
    let mut contents = String::new();
    buff_read.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn handling_arguments() -> Result<String, Error>{
    if env::args().len() < 2 {
        return Err(Error::new(ErrorKind::Other,"too little arguments"));
    }

    let mut args = env::args();

    for index in 0..args.len() {
        println!("command: {}", args.nth(index).unwrap());
        if args.nth(index).unwrap() == "-file" {
            return Ok(args.nth(index).unwrap());
        }
    }
    
    Err(Error::new(ErrorKind::Other,"Unkown command"))
}


fn main() -> Result<(), Error> {

    let file_name = handling_arguments()?;

    println!("File name: {}", file_name);

    let content = read_file(&file_name).expect("Should return file content");
    

    println!("File: \n {}", content);

    Ok(())
}
