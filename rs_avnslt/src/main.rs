use avnslt::prompt_create_file;


fn main() {
    // TODO: Goal is to have smth like this:
    // ReplEdi::run(); in main
    
    let file = prompt_create_file();
    let _ = file.save_file();
}
