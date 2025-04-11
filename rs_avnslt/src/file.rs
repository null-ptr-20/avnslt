use std::fs;
use std::io::{Write, Result};
use std::ffi::OsString;
use std::path::{PathBuf, Path};

use crate::ui;
use crate::prompt_scan;



/*
* Specify a file to the user wants to create as a struct
* title: Title of document
* date: Date of document creation
* body: Main body of the document (how to incorporate enter key?)
* TODO: add in access field for the edit file module.
*/
pub struct CFile {
   pub title: String,
   pub date: String,
   pub body: Vec<String>,
   pub access: bool, //RO RW maybe
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
* Vector is used to save multi line body sections
*/
impl CFile {
    pub fn build(title: String, date: String, body: Vec<String>, access: bool) -> Self {
        Self {
            title,
            date,
            body,
            access,
        }
    }
    
    // TODO: save value increases every subsequent save EO(i)
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
        for line in file_body.iter() {
                writeln!(&file, "{}{}", ui::TWOTAB, line)?;
        }
        writeln!(&file, "-- EO(Original)")?;
        //writeln!(&file, "-- EO({})", smth)?;

        Ok(())
    }

    /*
    // NOTE: read write save based on access flag
    //       but is this already accomplished with save_file 
    //       since create is used not create_new?
    pub fn edit_file(&self, path: OsString) -> Result<()> {
        let path_echeck: bool = Path::new(&path).exists();

        if path_echeck {
            let file_contents = fs::read_to_string(&path)?;
            println!("{}", file_contents);
        } else {
            eprintln!("Error! @ -> The file path specified does not exist");
        }

        Ok(())
    }
    */

}

// Trait implementation
impl Summary for CFile {
    fn summarize(&self) -> String {
        format!("Summary:\nTitle: {}\nDate saved: {}, Access: {}", self.title, self.date, self.access)
    }
}
