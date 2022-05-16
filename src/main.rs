use std::io::{Error, ErrorKind};

mod helpers;
mod core;


fn remove_file_name_from_path(path: &str) -> Result<String, Error> {
   let mut file_name_end = 0;
    
   for index in (0..path.len()).rev() {
        if path.as_bytes()[index] as char == '/' {
            file_name_end = index; 
            break;
        }

   }  

   if file_name_end == 0 {
       return Err(Error::new(ErrorKind::Other,"Cannot format path"));
   }

   Ok(path[0..file_name_end].to_string())
}

fn main() -> Result<(), Error> {
    let arguments = helpers::EnvHelper::get_arguments()?;
    let (file_content, file_path) = helpers::FileHelper::get_content(&arguments.file_name, &arguments.filter_path)?;

    let mut vars = core::Parser::parse_constructor(&file_content)?;
    
    core::Parser::get_imports_for_vars(&file_content, &mut vars, &file_path);

    let formated_vars_to_write = core::Writer::write_test_file(&vars)?;

    let file_path_formated = remove_file_name_from_path(&file_path)?;
    helpers::FileHelper::write_on_file(formated_vars_to_write, &file_path_formated)?;


    Ok(())
}
