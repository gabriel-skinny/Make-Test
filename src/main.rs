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

    let argv: Vec<String> = env::args().collect();

    for index in 0..argv.len() {
        if argv[index] == "-file" {
            return Ok(argv.into_iter().nth(index + 1).unwrap());
        }
    }
    
    Err(Error::new(ErrorKind::Other,"Unkown command"))
}

fn find_word_in_string(word: &str, content: &str) -> usize {
  let limit_count = 0;

   for letter in content.chars() {
       for limit_letter in word {
            if limit_letter == letter {

            }
       }
   }
    
}

fn main() -> Result<(), Error> {

    let file_name = handling_arguments()?;
    let content = read_file(&file_name).expect("Should return file content");

    find_word_in_string("function", &content);

    println!("File: \n {}", content);

    Ok(())
}
