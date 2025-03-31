use avnslt::prompt_create_file;


/*
* TODO: Goal is to have something like this:
* ReplEdi::run(); in main
*/
fn main() {
    let file = prompt_create_file();
    let _ = file.save_file();
}
