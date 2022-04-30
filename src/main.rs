use std::fs::File;
use std::io::{BufReader, Read, Error, ErrorKind};
use std::env;
use std::process::Command;
use std::str;

struct Var{
    className: String,
    instanciated_name: String,
    interface: String,
}

impl Default for Var {
    fn default() -> Self {
        Self { 
            className: String::new(),
            instanciated_name: String::new(),
            interface: String::new()
        }
    }
}

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
         return Ok(index + 1);            
       }
   }

    Err(Error::new(ErrorKind::Other, "Word not found"))   
}

fn get_constructor_lines(content: &str) -> Result<Vec<String>, Error> {
    let contructor_init = find_word_in_string("constructor", content)?;
    let mut lines = Vec::new();
    let mut line = String::new();

    for index in contructor_init..content.len() {
        let word = content.as_bytes()[index] as char;

        if word == '{' {
            return Ok(lines);
        }

        if word != ',' {
            line.push(word);
        } else {
            lines.push(line.clone());
            line.clear();
        }

    }

    Err(Error::new(ErrorKind::Other, "Constructor delimiter not found"))   
}

fn get_var_names(constructor_lines: &Vec<String>) -> Result<Vec<Var>, Error>{
    let variables = vec!(Var::Default); 

    for line in constructor_lines {
        let limitWord = if line.contains("private readonly") { "private readonly"} else { "private"};

        let init_var = find_word_in_string(&limitWord, line);
        for word_index in init_var..line.len() {
            let word = line.as_bytes()[word_index] as char;
            let instanciated_name = String::new();
            if word != ':' {
                instanciated_name.push(word);
            }

            variables.push(Var {
             instanciated_name 
            })
        }
    }
    
    Ok(variables)
}

fn main() -> Result<(), Error> {
    let file_name = handling_arguments()?;
    let file_path = find_file(&file_name);
    let content = read_file(&file_path)?;

    let foundIndex = find_word_in_string("constructor", &content)?;

    let lines = get_constructor_lines(&content)?;


    println!("Lines : {:?}", lines);

    println!("Index: \n {}", foundIndex);
    println!("File: \n {}", content);
    println!("File in index: \n {}", content.into_bytes()[foundIndex] as char);

    Ok(())
}
