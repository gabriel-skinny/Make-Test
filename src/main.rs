use std::fs::File;
use std::io::{BufReader, Read, Error, ErrorKind};
use std::env;
use std::process::Command;
use std::str;

fn read_file(file_path: &str) -> Result<String, Error>{
    let file = File::open(file_path.trim())?;
    
    let mut buffer_read = BufReader::new(file); 
    let mut contents = String::new();

    buffer_read.read_to_string(&mut contents)?;
    
    Ok(contents)
}

fn find_file(file_name: &str) -> String {
   let find_file = Command::new("/bin/find")
                            .arg("-name")
                            .arg(file_name)
                            .output()
                            .expect("Could found file"); 

   str::from_utf8(&find_file.stdout).unwrap().to_string()
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

fn find_word_in_string(word: &str, content: &str) -> Result<usize, Error> {
  let mut limit_count = 0;

   for index in 0..content.len() {
       if word.as_bytes()[limit_count] as char == content.as_bytes()[index] as char {
            limit_count += 1;
       } else {
            limit_count = 0;
       }

       if limit_count >= word.len() {
         return Ok(index);            
       }
   }

    Err(Error::new(ErrorKind::Other, "Word not found"))   
}

fn main() -> Result<(), Error> {
    let file_name = handling_arguments()?;
    let file_path = find_file(&file_name);
    let content = read_file(&file_path)?;

    let foundIndex = find_word_in_string("function", &content)?;


    println!("Index: \n {}", foundIndex);
    println!("File: \n {}", content);
    println!("File in index: \n {}", content.into_bytes()[7] as char);

    Ok(())
}
