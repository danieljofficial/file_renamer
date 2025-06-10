#[derive(Debug, Default)]
pub struct ListOptions {
   pub recursive: bool,
   pub include_hidden: bool,
   pub include_directories: bool,
   pub extensions: Option<Vec<String>>,
}