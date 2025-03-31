use std::fs;
use std::io::{Write, Result};
use std::ffi::OsString;

use crate::prompt_scan;

/*
* Specify a file to the user wants to create as a struct
* title: Title of document
* date: Date of document creation
* body: Main body of the documnet (how to incorporate enter key?)
*/
pub struct File {
   pub title: String,
   pub date: String,
   pub body: String,
}

/*
* Trait that can be used for other structs in the future (?) 
* Use of this trait on line 53
*/
pub trait Summary {
    fn summarize(&self) -> String;
}

/*
* Implementation of the File struct
* build: Create the struct instance
* save_file: Save the document the user creates in the terminal into an actual file
* TODO: [save_file] Add some logic to ensure file extension (.txt for now) 
*/
impl File {
    pub fn build(title: String, date: String, body: String) -> Self {
        Self {
            title,
            date,
            body,
        }
    }
    
    pub fn save_file(&self) -> Result<()> {
        let file_name = OsString::from(prompt_scan("Please enter file name: ").trim());
        let file_path = fs::File::create(file_name).expect("Cannot create file!!");
        writeln!(&file_path, "Title: {}", self.title)?;
        writeln!(&file_path, "Date: {}", self.date)?;
        writeln!(&file_path, "Body: {}", self.body)?;
        Ok(())
    }
}

// Trait implementation
impl Summary for File {
    fn summarize(&self) -> String {
        format!("Summary:\nTitle: {}\nDate saved: {}", self.title, self.date)
    }
}
