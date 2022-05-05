use std::io::{Error, ErrorKind};
use std::str;

#[derive(Debug)]
#[allow(unused_variables, dead_code)]
pub struct Var {
    class_name: String,
    instanciated_name: String,
    interface: String,
    is_sut: bool,
}

pub fn parse_constructor(file_content: &str) -> Result<Vec<Var>, Error> {
   let constructor_lines = get_constructor_lines(file_content)?;

   let mut var_names = get_var_names(&constructor_lines)?;
   let var_sut = get_sut(file_content)?;
   var_names.push(var_sut);

   Ok(var_names)
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

    Err(Error::new(ErrorKind::Other, format!("Word not found in file: '{}'", word)))   
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

fn get_sut(content: &str) -> Result<Var, Error> {
   let init_limit = find_word_in_string("class", &content)? + 1;
   let mut final_sut_name_limit: usize;
   let mut sut_interface: String;
   let mut sut_name;

   match find_word_in_string("implements", &content) {
       Ok(interface_limit_end) => { 
           final_sut_name_limit = interface_limit_end - "implements".len() - 1;
           sut_name = content[init_limit..final_sut_name_limit].trim().to_string();
           sut_interface = String::new();
           for word_index in interface_limit_end + 1..content.len() {
               if content.as_bytes()[word_index] as char != '{' && content.as_bytes()[word_index] as char != ' ' {
                  sut_interface.push(content.as_bytes()[word_index] as char); 
               }else {
                   break;
               }
           }
       }
       Err(error) => {
            sut_name = String::new();
            for word_index in init_limit..content.len() {
                if content.as_bytes()[word_index] as char != ' ' {
                   sut_name.push(content.as_bytes()[word_index] as char); 
                }else {
                    break;
                }
            } 
            sut_interface = sut_name.clone();
       }
   }

   let instanciated_name = sut_name.clone()[0..1].to_lowercase() + &sut_name[1..];

   Ok(Var {
    class_name: sut_name,
    instanciated_name,
    interface: sut_interface,
    is_sut: true
   })
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
                    interface: real_object_name.clone(), 
                    is_sut: false
                })
            }
        }
    }
    
    Ok(variables)
}
