use std::io::{Error, ErrorKind};
use std::env;
use std::str;
mod helpers;

#[derive(Debug)]
#[allow(unused_variables, dead_code)]
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
        let mut class_name = String::new();

        if line.contains("Inject") {
           let mut start = false;

            for word in line.chars() {
                if word == ')' {start = false}

                if start {
                   class_name.push(word);
                }

                if word == '('  { start = true}
            }

        }

        let limit_word = if line.contains("private readonly") { "private readonly" } else { "private" };

        let init_var = find_word_in_string(&limit_word, &line)?;
        let mut instanciated_name = String::new();

        for word_index in init_var..line.len() {
            let word = line.as_bytes()[word_index] as char;
            if word != ':' {
                instanciated_name.push(word);
            } else {
                let real_object_name = line[word_index + 1..line.len()].trim().to_string();

                variables.push(Var {
                    instanciated_name: instanciated_name.clone().trim().to_string() ,
                    class_name: if class_name.is_empty() { real_object_name.clone()} else{ class_name.clone()}, 
                    interface: real_object_name.clone() 
                })
            }
        }
    }
    
    Ok(variables)
}

fn main() -> Result<(), Error> {
    let file_name = handling_arguments()?;
    let content = helpers::FileHelper::get_content(&file_name)?;

    let lines = get_constructor_lines(&content)?;

    let var_names = get_var_names(&lines);

    println!("Var name: {:?}", var_names);

    println!("File: \n {}", content);

    Ok(())
}
