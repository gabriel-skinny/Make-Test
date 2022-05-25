use std::io::{Error, ErrorKind};

use crate::helpers::Utils;

pub fn make(content: &str) {
   let class_lines = get_class_lines(content).unwrap(); 
   let functions = get_functions(&class_lines).unwrap();
   let arguments = get_arguments(&functions).unwrap();
   println!("Lines: \n {:?}", class_lines);
   println!("Functions: \n {:?}", functions);
   println!("Arguments: \n {:?}", arguments);
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
    let mut copy_function = true;
    for line in class_lines {
        if line.contains("{") {
            bracketsCount += 1;
        }

        if line.contains("}") && bracketsCount == 2 {
            copy_function = true;    
        }

        if line.contains("}") {
            bracketsCount -= 1;
        }


        if bracketsCount == 2 && !line.contains("=") && copy_function {
            functions.push(line.clone());  
            copy_function = false;
        }
    }

    if functions.len() < 1 { 
        return Err(Error::new(ErrorKind::Other, "No functions found")); 
    }
        
    Ok(functions)
}

fn get_arguments(functions: &Vec<String>) -> Result<Vec<String>, Error>{
    let mut argument = String::new();
    let mut arguments = Vec::new();


    for function in functions {
        if function.contains("()") {
            continue;
        } 

        let mut start = false;
        for word in function.chars() {
            if word == ')' {
                start = false;
                arguments.push(argument.clone());
                argument.clear();
            }

            if start {
                argument.push(word);
            }

            if word == '(' {
                start = true;
            }
        }


    }

    Ok(arguments)
}
