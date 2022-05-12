use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::process::Command;
use std::str;
use dialoguer::MultiSelect;
use std::fs;
use std::io::Write;

pub fn get_content(file_name: &str) -> Result<(String, String), Error> {
    let file_path = find_file(file_name)?;    

    Ok((read_file(&file_path)?, file_path))
}


fn find_file(file_name: &str) -> Result<String, Error> {
    let founded_files = Command::new("/bin/find")
        .arg("src/") 
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg(format!("{}*", file_name))
        .output()?;

    let founded_files_string = str::from_utf8(&founded_files.stdout)
        .unwrap()
        .to_string();

    if founded_files_string == "" {
        return Err(Error::new(ErrorKind::Other, format!("File not found. Motive: {}", str::from_utf8(&founded_files.stderr).unwrap())))   
    }

    if !founded_files_string.trim().contains('\n') {
        return Ok(founded_files_string);
    }

    let files = format_multiple_files(founded_files_string);
    chose_one_file(files)
}

fn format_multiple_files(files: String) -> Vec<String>{
    let mut files_result = Vec::new();
    let mut file_to_push = String::new();
    for word in files.chars() {
        if word != '\n' {
            file_to_push.push(word); 
        } else {
            files_result.push(file_to_push.clone());
            file_to_push.clear();
        }
    }

    files_result
}

fn chose_one_file(file_options: Vec<String>) -> Result<String, Error> {
    let chosen = MultiSelect::new().items(&file_options).interact()?;

    if chosen.len() < 1 {
        return Err(Error::new(ErrorKind::Other, "Should select a file"))   
    }

    return Ok(file_options[chosen[0]].clone());
}

fn read_file(file_path: &str) -> Result<String, Error>{
    let file = File::open(file_path.trim())?;

    let mut buffer_read = BufReader::new(file); 
    let mut contents = String::new();

    buffer_read.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn write_on_file(content: String, path: &str) -> Result<(), Error> {
    let to_write_path = format!("{}/tests/test.spec.ts", path);
    let test_dir = format!("{}/tests", path);

    if let Err(error) = fs::read_dir(&test_dir) {
        match error.kind() {
            ErrorKind::NotFound => {
                fs::create_dir(test_dir)?; 
                println!("Creating test directory...");
            }
            other_error => {
                return Err(Error::new(ErrorKind::Other, format!("Error finding test_dir {:?}", other_error)));   
            }
        }
    }


    let mut file_to_write = File::create(&to_write_path)?; 
    file_to_write.write_all(&content.as_bytes())?;    

    Ok(())
}
