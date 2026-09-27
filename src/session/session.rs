use std::path::PathBuf;
use crate::Edit;

#[derive(Debug)]
pub struct Session{
    path:PathBuf,
    edit:Edit
}

impl Session {
    pub fn new(path:PathBuf, edit:Edit)->Self{
        Self {
            path, 
            edit 
        }
    }
}