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

fn handling_arguments() -> Result<(), Error>{
    if env::args().len() > 2 {
        return Err(Error::new(ErrorKind::Other,"too little arguments"));
    }

    for arg in env::args() {
        if arg == "-file" {
            return Ok(());
        }
    }
    
    Err(Error::new(ErrorKind::Other,"Unkwung command"))
}


fn main() {
    let content = read_file("./teste.txt").expect("Should return file content");
    
    handling_arguments();

    println!("File: \n {}", content);
}
