pub mod FileHelper {
    use std::fs::File;
    use std::io::{BufReader, Error, ErrorKind, Read};
    use std::process::Command;
    use std::str;
    use dialoguer::MultiSelect;

    pub fn read_file(file_path: &str) -> Result<String, Error>{
        let file = File::open(file_path.trim())?;
        
        let mut buffer_read = BufReader::new(file); 
        let mut contents = String::new();

        buffer_read.read_to_string(&mut contents)?;
        
        Ok(contents)
    }

    pub fn find_file(file_name: &str) -> Result<String, Error> {
       let founded_files = Command::new("/bin/find")
                                .arg("src/") 
                                .arg("-type")
                                .arg("f")
                                .arg("-name")
                                .arg(format!("{}*", file_name))
                                .output()?;
    
       let founded_files_string = str::from_utf8(&founded_files.stdout).unwrap().to_string();

    

       let files = handle_multiple_files(founded_files_string);
       let file_chosed = chose_one_file(files)?;
       Ok(file_chosed)
    }

    fn handle_multiple_files(files: String) -> Vec<String>{
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
}
