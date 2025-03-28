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
}
