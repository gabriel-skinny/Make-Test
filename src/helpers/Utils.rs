use std::io::{Error, ErrorKind};

pub fn find_word_in_string(word: &str, content: &str) -> Result<usize, Error> {
    let mut limit_count = 0;

    for index in 0..content.len() {
        if word.as_bytes()[limit_count] as char == content.as_bytes()[index] as char {
            limit_count += 1;
        } else {
            limit_count = 0;
        }

        if limit_count >= word.len() {
            return Ok(index + 1);            
        }
    }

    Err(Error::new(ErrorKind::Other, format!("Word not found in file: '{}'", word)))   
}


pub fn remove_file_name_from_path(path: &str) -> Result<String, Error> {
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
