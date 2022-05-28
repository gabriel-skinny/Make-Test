use std::io::{Error, ErrorKind};
use crate::core::Parser;

pub fn write_test_file(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    let formated_imports = format_imports(vars)?;
    let sut_class_name = get_sut_class_name(vars)?;
    let spies = make_spies(vars);
    let injections = inject_dependencies_on_sut(vars)?; 
    let assignments = making_assignments(vars);
    let typed_vars = typing_vars(vars);


    Ok(make_test_suit(formated_imports, sut_class_name, spies, typed_vars, assignments, injections))
}

fn get_sut_class_name(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    for var in vars {
        if var.is_sut { return Ok(var.class_name.clone());}
    }

    Err(Error::new(ErrorKind::Other, "Sut not found"))
}


fn format_imports(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    let mut formated_imports = String::new();
    for var in vars {
        match &var.import { 
            Some(import) => formated_imports.push_str(&format!("{}\n", import)),
            None => return Err(Error::new(ErrorKind::Other, format!("Could find import for {}", var.class_name)))
        }
    }

    Ok(formated_imports) 
}

fn make_spies(vars: &Vec<Parser::Var>) -> String {
    let mut spies = String::new();

    for var in vars {
        if !var.is_sut {
            spies.push_str(&format!("class {}Spy implements {} {{}}\n\n", var.class_name, var.interface));
        } 
    };     
    
    spies
}

fn inject_dependencies_on_sut(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    let mut sut_injection:Option<String> = None;  
    let mut dependencies_format = String::new();

    for var in vars {
        if !var.is_sut {
            dependencies_format.push_str(&format!("\t\t\t{},\n", var.instanciated_name));
        } else {
            sut_injection = Some(format!("\t\tsut = new {}", var.class_name)); 
        }

    }

    if let Some(sut_to_inject) = sut_injection {
        let injected_dependencies = format!("{}(\n{}\t\t)", sut_to_inject, dependencies_format);

        return Ok(injected_dependencies); 
    }

    Err(Error::new(ErrorKind::Other, "Sut not found"))   
}

fn making_assignments(vars: &Vec<Parser::Var>) -> String {
    let mut all_assignments = String::new();
    for var in vars {
        if !var.is_sut {
            all_assignments.push_str(&format!("\t\t{} = new {}();\n", var.instanciated_name, var.class_name)); 
        }
    }


    all_assignments 
}

fn typing_vars(vars: &Vec<Parser::Var>) -> String {
    let mut all_typing = String::new();
    for var in vars {
        if var.is_sut {
            all_typing.push_str(&format!("\tsut: {};\n", var.interface));
        } else {
            all_typing.push_str(&format!("\t{}: {};\n", var.instanciated_name, var.interface)); 
        }
    }


    all_typing 
}

fn make_test_suit(imports: String, sut_class_name: String, spies: String, typed_vars: String, assignments: String, injections: String) -> String {
   format!("{}

{}

describe('{}'), () => {{
{}

  beforeEach(() => {{)
{}

{}
   }})   
}}", imports, spies, sut_class_name, typed_vars, assignments, injections)
}








