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
    let file_name = helpers::EnvHelper::handling_arguments()?;
    let (file_content, file_path) = helpers::FileHelper::get_content(&file_name)?;

    let vars = core::Parser::parse_constructor(&file_content)?;
    let formated_vars_to_write = core::Writer::write_test_file(&vars)?;

    let file_path_formated = remove_file_name_from_path(&file_path)?;
    helpers::FileHelper::write_on_file(formated_vars_to_write, &file_path_formated)?;

    println!("File: \n {}", file_content);
    println!("Var name: {:?}", vars);

    Ok(())
}
