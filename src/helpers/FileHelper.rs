pub mod FileHelper {
    use std::fs::File;
    use std::io::{BufReader, Error, Read};
    use std::process::Command;
    use std::str;

    pub fn read_file(file_path: &str) -> Result<String, Error>{
        let file = File::open(file_path.trim())?;
        
        let mut buffer_read = BufReader::new(file); 
        let mut contents = String::new();

        buffer_read.read_to_string(&mut contents)?;
        
        Ok(contents)
    }

    pub fn find_file(file_name: &str) -> String {
       let find_file = Command::new("/bin/find")
                                .arg("-name")
                                .arg(file_name)
                                .output()
                                .expect("Could found file"); 

       str::from_utf8(&find_file.stdout).unwrap().to_string()
    }

}
