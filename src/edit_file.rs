#[derive(Debug, Clone)]
pub struct EditFile{
    old:String,
    new:String,
    state:OpState,
    conflict:Conflict
}

impl EditFile {
    pub fn new_pending(old:impl Into<String>, new:impl Into<String>, is_safe:bool)->Self{
        Self { 
            old:old.into(), 
            new:new.into(), 
            state: OpState::Pending, 
            conflict: if is_safe {Conflict::Safe} else {Conflict::Conflict} 
        }
    }
}

#[derive(Debug, Clone)]
pub enum OpState {
    Pending,
    Planned,
    Applied,
    RolledBack,
    Failed,
}

#[derive(Debug, Clone)]
pub enum Conflict {
    Safe,
    Conflict, 
    Cycle,
    Chain
}