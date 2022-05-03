use std::io::Error;

mod helpers;
mod core;

fn main() -> Result<(), Error> {
    let file_name = helpers::EnvHelper::handling_arguments()?;
    let file_content = helpers::FileHelper::get_content(&file_name)?;
    let var_names = core::Parser::parse_constructor(&file_content)?;

    println!("Var name: {:?}", var_names);

    println!("File: \n {}", file_content);

    Ok(())
}
