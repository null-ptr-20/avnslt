use std::fs;
use std::io::{Write, Result};
use std::ffi::OsString;

use crate::prompt_scan;

pub struct File {
   pub title: String,
   pub date: String,
   pub body: String,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl File {
    pub fn build(title: String, date: String, body: String) -> Self {
        Self {
            title,
            date,
            body,
        }
    }
    
    //TODO: Add some checks for file type
    
    pub fn save_file(&self) -> Result<()> {
        let file_name = OsString::from(prompt_scan("Please enter file name: ").trim());
        let file_path = fs::File::create(file_name).expect("Cannot create file!!");
        writeln!(&file_path, "Title: {}", self.title)?;
        writeln!(&file_path, "Date: {}", self.date)?;
        writeln!(&file_path, "Body: {}", self.body)?;
        Ok(())
    }
}

impl Summary for File {
    fn summarize(&self) -> String {
        format!("Summary:\nTitle: {}\nDate saved: {}", self.title, self.date)
    }
}
