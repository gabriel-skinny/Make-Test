use std::io::{Error, ErrorKind};
use std::str;
use crate::helpers::Utils;

#[derive(Debug)]
pub struct Var {
    pub class_name: String,
    pub instanciated_name: String,
    pub interface: String,
    pub is_sut: bool,
    pub import: Option<String>,
}

pub fn parse_constructor(file_content: &str) -> Result<Vec<Var>, Error> {
    let constructor_lines = get_constructor_lines(file_content)?;

    let mut var_names = get_var_names(&constructor_lines)?;
    let var_sut = get_sut(file_content)?;
    var_names.push(var_sut);

    Ok(var_names)
}

fn get_constructor_lines(content: &str) -> Result<Vec<String>, Error> {
    let contructor_init = Utils::find_word_in_string("constructor", content)?;
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

        let init_var = Utils::find_word_in_string(&limit_word, &line)?;
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
                    is_sut: false,
                    import: None 
                })
            }
        }
    }

    Ok(variables)
}

fn get_sut(content: &str) -> Result<Var, Error> {
    let init_limit = Utils::find_word_in_string("class", &content)? + 1;
    let final_sut_name_limit: usize;
    let mut sut_interface: String;
    let mut sut_name;

    match Utils::find_word_in_string("implements", &content) {
        Ok(interface_limit_end) => { 
            final_sut_name_limit = interface_limit_end - "implements".len() - 1;
            sut_name = content[init_limit..final_sut_name_limit].trim().to_string();
            sut_interface = String::new();
            for word_index in interface_limit_end..content.len() {
                if content.as_bytes()[word_index] as char != '{' && content.as_bytes()[word_index] as char != '\n' {
                    sut_interface.push(content.as_bytes()[word_index] as char); 
                }else {
                    break;
                }
            }
        }
        Err(_error) => {
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
        interface: sut_interface.trim().to_string(),
        is_sut: true,
        import: None
    })
}

pub fn get_imports_for_vars(content: &str, vars: &mut Vec<Var>, file_path: &str) {
    let mut content_line: Vec<&str> = content.split("\n").collect();

    for line in content_line.iter_mut() {
        for var in vars.iter_mut() {
            if var.is_sut {
                var.import = Some(format!("import {{ {} }} from '{}';", var.class_name, file_path.trim()));
                break;
            }

            if line.contains(&var.class_name) && line.contains("import") {
               var.import = Some(line.to_string()); 
               break;
            }
        }
    }
    
}
