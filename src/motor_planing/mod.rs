use std::fs::ReadDir;

pub fn get_name_fs_entry(entrys: ReadDir)->Result<Vec<String>, std::io::Error>{
    let mut entrys_names = Vec::new();
    for dir in entrys{
        let dir = dir?;
        let name = match dir.file_name().to_str(){
            Some(name) => name.into(),
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Nombre no disponible"))
        };
        entrys_names.push(name);
    }
    Ok(entrys_names)
}