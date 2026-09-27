#[derive(Debug)]
pub struct EditFile{
    old:String,
    new:String,
    state:OpState,
    conflict:Conflict
}

#[derive(Debug)]
pub enum OpState {
    Pending,
    Planned,
    Applied,
    RolledBack,
    Failed,
}

#[derive(Debug)]
pub enum Conflict {
    Safe,
    Conflict, 
    Cycle,
    Chain
}