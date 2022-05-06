use std::io::Error;

mod helpers;
mod core;

fn main() -> Result<(), Error> {
    let file_name = helpers::EnvHelper::handling_arguments()?;
    let file_content = helpers::FileHelper::get_content(&file_name)?;
    let vars = core::Parser::parse_constructor(&file_content)?;

    let formated_vars_to_write = core::Writer::write_test_file(&vars)?;

    helpers::FileHelper::write_on_file(formated_vars_to_write, "src/results/")?;

    println!("Var name: {:?}", vars);

    println!("File: \n {}", file_content);

    Ok(())
}
