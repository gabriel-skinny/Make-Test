use std::io::{Error, ErrorKind};
use std::env;
use std::str;
mod helpers;

#[derive(Debug)]
struct Var{
    class_name: String,
    instanciated_name: String,
    interface: String,
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
    let mut variables = Vec::new(); 

    for line in constructor_lines {
        let limit_word = if line.contains("private readonly") { "private readonly" } else { "private" };

        let init_var = find_word_in_string(&limit_word, &line)?;
        let mut instanciated_name = String::new();

        for word_index in init_var..line.len() {
            let word = line.as_bytes()[word_index] as char;
            if word != ':' {
                instanciated_name.push(word);
            } else {
                variables.push(Var {
                    instanciated_name: instanciated_name.clone() ,
                    class_name: "Billing".to_string(),
                    interface: "IBilling".to_string()
                })
            }
        }
    }
    
    Ok(variables)
}

fn main() -> Result<(), Error> {
    let file_name = handling_arguments()?;
    let file_path = helpers::FileHelper::FileHelper::find_file(&file_name)?;
    let content = helpers::FileHelper::FileHelper::read_file(&file_path)?;

    let foundIndex = find_word_in_string("constructor", &content)?;

    let lines = get_constructor_lines(&content)?;

    let var_names = get_var_names(&lines);

    println!("Var name: {:?}", var_names);
    println!("Lines : {:?}", lines);

    println!("Index: \n {}", foundIndex);
    println!("File: \n {}", content);
    println!("File in index: \n {}", content.into_bytes()[foundIndex] as char);

    Ok(())
}
