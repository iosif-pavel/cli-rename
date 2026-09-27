use crate::edit_file::EditFile;

#[derive(Debug)]
pub enum Edit{
    All,
    Only(Vec<EditFile>)
}