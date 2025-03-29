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
    
    pub fn save_file(&self) -> Result<()> {
        let file_name = OsString::from(prompt_scan("Please enter file name: ").trim());
        let file_path = fs::File::create(file_name).expect("Cannot create file!!");
        writeln!(&file_path, "Title:\n{}", self.title)?;
        writeln!(&file_path, "Date:\n{}", self.date)?;
        writeln!(&file_path, "Body:\n{}", self.body)?;
        Ok(())
    }
}

impl Summary for File {
    fn summarize(&self) -> String {
        format!("The text file is titled: {}\nWritten on date: {}", self.title, self.date)
    }
}
