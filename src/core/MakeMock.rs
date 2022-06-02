use std::io::{Error, ErrorKind};
use crate::helpers::Utils;
use crate::helpers::FileHelper;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Argument { 
   argument_name: String,
   interface: String,
   mock: String,
   path_to_interface: Option<String>
}

enum JavaScripTypes {
    Date,
    Number,
    NumberArr,
    Str,
    StrArr,
    Any,
    Boolean,
    Object(Vec<JavaScripTypes>)
}


pub fn make(content: &str) -> Result<Vec<Argument>, Error> {
    let class_lines = get_class_lines(content)?; 
    let functions = get_functions(&class_lines)?;
    let mut arguments = get_arguments(&functions)?;

    create_mock(&mut arguments);
    get_imports_for_arguments(&content ,&mut arguments);

    println!("Functions: \n {:?}", functions);
    println!("Arguments: \n {:?}", arguments);

    Ok(arguments)
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

fn get_arguments(functions: &Vec<String>) -> Result<Vec<Argument>, Error>{
    let mut argument_name = String::new();
    let mut arguments = Vec::new();
    let mut interface = String::new();

    for function in functions {
        if function.contains("()") {
            continue;
        } 

        let mut read_argument_name = false;
        let mut read_interface = false;
        for word in function.chars() {
            if word == ')' {
                read_argument_name = false;
                read_interface = false;
                arguments.push(
                    Argument {
                        argument_name: argument_name.clone(),
                        interface: interface.clone(),
                        mock: String::new(),
                        path_to_interface: None,
                    }
                    );
                argument_name.clear();
                interface.clear();
            }
            if read_interface && word != ' ' {
                interface.push(word);
            }

            if word == ':' {
                read_interface = true;
                read_argument_name = false;
            }

            if read_argument_name {
                argument_name.push(word);
            }


            if word == '(' {
                read_argument_name = true;
            }
        }


    }

    Ok(arguments)
}

fn create_mock(arguments: &mut Vec<Argument>)  {
   for argument in arguments.iter_mut() {
    if let Some(argument_type) = enumerate_arguments(&argument.interface) {
      match argument_type {  
        JavaScripTypes::Number => argument.mock = "123".to_owned(),
        JavaScripTypes::NumberArr => argument.mock = "[1242, 1923]".to_owned(),
        JavaScripTypes::StrArr => argument.mock = "['teste1', 'teste2']".to_owned(),
        JavaScripTypes::Str => argument.mock = "teste".to_owned(),
        JavaScripTypes::Date => argument.mock = "2022-04-03T03:00:00".to_owned(),
        JavaScripTypes::Any => argument.mock = "any".to_owned(),
        _ => argument.mock = "".to_owned(), 
      }
    };
   } 
}

fn enumerate_arguments(interface_type: &str) -> Option<JavaScripTypes> {
    //Suport to enum in typescript like "banana" || "abacaxi". Use default of one option of enum

    match interface_type{
        "number" => Some(JavaScripTypes::Number),
        "number[]" => Some(JavaScripTypes::NumberArr),
        "string[]" => Some(JavaScripTypes::StrArr),
        "string" => Some(JavaScripTypes::Str),
        "Date" => Some(JavaScripTypes::Date),
        "boolean" => Some(JavaScripTypes::Boolean),
        "any" => Some(JavaScripTypes::Any),
        _ => None 
    } 
}


fn get_imports_for_arguments(content: &str, arguments: &mut Vec<Argument>) -> Result<(), Error> {
    let content_line: Vec<&str> = content.split("\n").collect();

    for argument in arguments.iter_mut() {
        if argument.mock != "" {
            continue;
        }

        for line in &content_line {
            if line.contains(&argument.interface) && line.contains("import") {
                let path = get_path_from_import(line); 
                argument.mock = resolve_mock_to_import(path, &argument.interface)?;
                break;
            }
        }
    }

    Ok(())
}


fn get_path_from_import(import_line: &str) -> String {
    let mut start = false;
    let mut path = String::new();

    for word in import_line.chars() {
        if (word == '"' && start) || word == ';' {
            start = false;
        }

        if start {
            path.push(word);
        }

        if word == '"' || word == '\'' {
            start = true; 
        }
    }    

    path
}

fn resolve_mock_to_import(path: String, interfaceName: &str) -> Result<String, Error> {
    let file_content = FileHelper::read_file(&path)?;

    let content_lines = file_content.split("\n");     
    let mut interface_content: Vec<&str> = Vec::new();

    let mut start_copy = false;
    for line in content_lines {
        if line.contains(&interfaceName) && line.contains("interface") && line.as_bytes()[line.len() - 1] as char  == '{' {
            start_copy = true;
            continue;
        }

        if line.contains("}") {
            start_copy = false;
        }

        if start_copy {
           interface_content.push(line.trim()); 
        }
    }


    let interfaceMock = mock_interface_content(&interface_content);

    println!("Interface mock {:?}", interfaceMock);

    Ok("bala".to_owned())
}

fn mock_interface_content(content: &Vec<&str>) -> HashMap<String, String> {
    let mut interface_map = HashMap::new();

    for line in content {
       let mut key = String::new();
       let mut key_type = String::new();
       let mut start_key_copy = true;
       for word in line.chars() {
            if word == ':' {
                start_key_copy = false;
                continue;
            } 

            if word == ';' {
                interface_map.insert(key.clone(), key_type.clone());
                continue;
            }

            if start_key_copy {
              key.push(word);      
            }

            if !start_key_copy && word != ' ' {
                key_type.push(word);
            }

       } 
    }

    interface_map
}

