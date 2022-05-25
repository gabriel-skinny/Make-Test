use std::io::{Error, ErrorKind};

use crate::helpers::Utils;

pub fn make(content: &str) {
   let class_lines = get_class_lines(content).unwrap(); 
   let functions = get_functions(&class_lines).unwrap();
   println!("Lines: \n {:?}", class_lines);
   println!("Functions: \n {:?}", functions);
}

fn get_class_lines(content: &str) -> Result<Vec<String>, Error> {
    let contructor_init = Utils::find_word_in_string("class", content)?;
    let mut lines = Vec::new();
    let mut line = String::new();

    let mut start_copy = false;
    for index in contructor_init..content.len() {
        let word = content.as_bytes()[index] as char;

        if word == '{' {
            start_copy = true;
        }

        if start_copy {
            if word != '\n' {
                line.push(word);
            } else {
                if line.trim() != "" {
                    lines.push(line.clone().trim().to_owned());
                }
                line.clear();
            }
        }

    }

   Ok(lines) 
}

fn get_functions(class_lines: &Vec<String>) -> Result<Vec<String>, Error> {
    let mut functions = Vec::new();

    let mut bracketsCount = 0;
    let mut check_function = true;
    for line in class_lines {
        if line.contains("{") {
            bracketsCount += 1;
        }

        if line.contains("}") && bracketsCount == 2 {
            check_function = true;    
        }

        if line.contains("}") {
            bracketsCount -= 1;
        }


        if bracketsCount == 2 && !line.contains("=") && check_function {
            functions.push(line.clone());  
            check_function = false;
        }
    }

    if functions.len() < 1 { 
        return Err(Error::new(ErrorKind::Other, "No functions found")); 
    }
        
    Ok(functions)
}
