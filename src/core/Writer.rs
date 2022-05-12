use std::io::{Error, ErrorKind};
use crate::core::Parser;

pub fn write_test_file(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    let formated_imports = format_imports(vars)?;
    let injections = inject_dependencies_on_sut(vars)?; 
    let assignments = making_assignments(vars);
    let typed_vars = typing_vars(vars);

    println!("\n\nSut injection: \n {}\n\n", injections);
    println!("\n\nAssignemnets : \n {}\n\n", assignments);
    println!("\n\nImports : \n {}\n\n", formated_imports);

    Ok(make_test_suit(formated_imports, typed_vars, assignments, injections))
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

fn inject_dependencies_on_sut(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    let mut sut_injection:Option<String> = None;  
    let mut dependencies_format = String::new();

    for var in vars {
        if !var.is_sut {
            dependencies_format.push_str(&format!("\t{},\n", var.instanciated_name));
        } else {
            sut_injection = Some(format!("{} = new {}", var.instanciated_name, var.class_name)); 
        }

    }

    if let Some(sut_to_inject) = sut_injection {
        let injected_dependencies = format!("{}(\n{}\t)", sut_to_inject, dependencies_format);

        return Ok(injected_dependencies); 
    }

    Err(Error::new(ErrorKind::Other, "Sut not found"))   
}

fn making_assignments(vars: &Vec<Parser::Var>) -> String {
    let mut all_assignments = String::new();
    for var in vars {
        if var.is_sut {
            all_assignments.push_str(&format!("sut = new {}()\n", var.class_name));
        } else {
            all_assignments.push_str(&format!("{} = new {}()\n", var.instanciated_name, var.class_name)); 
        }
    }


    all_assignments 
}

fn typing_vars(vars: &Vec<Parser::Var>) -> String {
    let mut all_typing = String::new();
    for var in vars {
        if var.is_sut {
            all_typing.push_str(&format!("sut: {}\n", var.interface));
        } else {
            all_typing.push_str(&format!("{}: {}\n", var.instanciated_name, var.interface)); 
        }
    }


    all_typing 
}

fn make_test_suit(imports: String, typed_vars: String, assignments: String, injections: String) -> String {
   format!("{}
describe('sut_name'), () => {{
    {}
    beforeEach(() => {{)
        {}

        {}
     }})   
   }}", imports, typed_vars, assignments, injections)
}









