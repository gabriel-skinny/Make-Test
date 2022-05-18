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
                if line.trim() != "" && line != "{" {
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

    // To pick function the else methods has to find the end of the brackets with one open bracket
    // is found the sum of total brackes is sum to one, in the one the end of the bracket has to
    // match the function open bracket
    for line in class_lines {
        if line.contains("(") && line.contains(")") && line.as_bytes()[line.len() - 1] as char == '{' {
            if line.contains("async") {
                functions.push(line.clone());
            }
        }
    }

    if functions.len() < 1 { 
        return Err(Error::new(ErrorKind::Other, "No functions found")); 
    }
        
    Ok(functions)
}
