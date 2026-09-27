use std::path::PathBuf;
use crate::{Edit, session::Session};

pub struct SessionBuilder{
    path:Option<PathBuf>,
    edit:Option<Edit>
}

impl SessionBuilder {
    pub fn new()->Self{
        Self {
            path: None,
            edit: None
        }
    }
    pub fn set_path(&mut self, path:PathBuf){
        self.path = Some(path)
    }

    pub fn to_session(self)->Session{
        Session::new(
            self.path.expect("El path es nesesario"),
            self.edit.expect("No se definio el plan de edicion")
        )
    }
}