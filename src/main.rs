use std::io::{Error};

mod helpers;
mod core;

fn main() -> Result<(), Error> {
    let file_arguments = helpers::EnvHelper::get_arguments()?;
    let (file_content, file_path) = helpers::FileHelper::get_content(&file_arguments)?;

    let mut vars = core::Parser::parse_constructor(&file_content)?;
    
    core::Parser::get_imports_for_vars(&file_content, &mut vars, &file_path);

    core::MakeMock::make(&file_content)?;

    let formated_vars_to_write = core::Writer::write_test_file(&vars)?;

    let file_path_formated = helpers::Utils::remove_file_name_from_path(&file_path)?;
    helpers::FileHelper::write_on_file(formated_vars_to_write, &file_path_formated)?;


    Ok(())
}
