use std::io::{Error, ErrorKind};
use crate::core::Parser;

pub fn write_test_file(vars: &Vec<Parser::Var>) -> Result<String, Error> {
    let injections = inject_dependencies_on_sut(vars)?; 
    let assignments = making_assignments(vars);

    println!("\n\nSut injection: \n {}\n\n", injections);
    println!("\n\nAssignemnets : \n {:?}\n\n", assignments);

    Ok(injections)
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

fn making_assignments(vars: &Vec<Parser::Var>) -> Vec<String> {
    let mut all_assignments = Vec::new();
    for var in vars {
        let assignment;  
        if var.is_sut {
            assignment = format!("sut = new {}()", var.class_name);
        } else {
            assignment = format!("{} = new {}()", var.instanciated_name, var.class_name); 
        }

        all_assignments.push(assignment); 
    }


    all_assignments 
}

