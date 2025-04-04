use std::fs;
use std::io::{Write, Result};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::ui;
use crate::prompt_scan;

/*
* Specify a file to the user wants to create as a struct
* title: Title of document
* date: Date of document creation
* body: Main body of the document (how to incorporate enter key?)
*/
pub struct File {
   pub title: String,
   pub date: String,
   pub body: Vec<String>,
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
* PathBuf is used as a mutable path 
* Checks performed: If no extention -> auto saves as a .txt file
* TODO: [save_file] Find how to enable enter button for body portion so as to start new line.
*/
impl File {
    pub fn build(title: String, date: String, body: Vec<String>) -> Self {
        Self {
            title,
            date,
            body,
        }
    }
    
    pub fn save_file(&self) -> Result<()> {
        let file_name = OsString::from(prompt_scan("Please enter file name: ").trim());
        let mut file_path = PathBuf::from(&file_name);

        if file_path.extension().is_none() {
            eprintln!("There is no file extention..\nSaving as a txt file");
            file_path.set_extension("txt");
        }

        let file = fs::File::create(file_path).expect("Cannot create file!!");
        let file_body: &Vec<String> = &self.body;

        writeln!(&file, "Title: {}", self.title)?;
        writeln!(&file, "Date: {}", self.date)?;
        writeln!(&file, "Body:")?;
        for (i, line) in file_body.iter().enumerate() {
            if i < file_body.len() - 1 {
                writeln!(&file, "{}{}", ui::TWOTAB, line)?;
            } else {
                writeln!(&file, "-- EO(1)")?;
            }
        }

        Ok(())
    }
}

// Trait implementation
impl Summary for File {
    fn summarize(&self) -> String {
        format!("Summary:\nTitle: {}\nDate saved: {}", self.title, self.date)
    }
}
