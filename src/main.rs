mod utils;
mod edit_file;
mod session;
mod edit;
mod motor_planing;

pub use edit::Edit;

use utils::input_data;
use std::{fs::{DirEntry, metadata}, path::{Path, PathBuf}};

use crate::{edit_file::EditFile, motor_planing::get_name_fs_entry, session::{Session, SessionBuilder}, utils::{input_data_usize, work_to_fs_erros}};

fn main() {
    let params = make_session();
    println!("{:?}", params)
}

fn make_session()->Session{
    let mut params = SessionBuilder::new();

    let path = ask_path();
    params.set_path(path);

    params.to_session()
}
fn ask_path()->PathBuf{
    loop {
        let path = input_data("Ingrese el path de la carpeta");
        if !path.is_empty(){
            let path = Path::new(&path);
            match work_to_fs_erros(metadata(path)) { //Esto ya trabaja con el error internamente
                Some(metadata) => {
                    if metadata.is_dir(){
                        let list = match work_to_fs_erros(path.read_dir()){
                            Some(list) => list,
                            None => continue 
                        };
                        let list:Vec<Result<DirEntry, std::io::Error>> = list.collect();
                        println!("Dirctorio encontrado, {} entradas encontradas", list.len());
                        break path.to_path_buf();
                    }else{
                        println!("Es un archivo");
                    }
                }
                None => continue
            }
        }else {
            println!("Path vacio, intentalo de nuevo")
        }
    }
}
fn ask_edit_mode(path:PathBuf)->Edit{
    if ask_editing_mode() {
        Edit::All
    }else{
        let quantity_edit = ask_quantity_a_edit();

        let mut olds_names = Vec::new();
        let mut news_names = Vec::new();
        let in_disk_names = match work_to_fs_erros(get_name_fs_entry(path.read_dir().expect("Error al procesar el directorio"))){
            Some(idn) => idn,
            None => panic!("Estas seguro que elejiste el directorio correcto?")
        };
        let mut edit_files = Vec::new();

        for _ in 0..quantity_edit{
            for (old, new, edit) in ask_edit_file(&olds_names, &news_names, &in_disk_names){
                olds_names.push(old);
                news_names.push(new);
                edit_files.push(edit);
            }
        }
        Edit::Only(edit_files)
    }
}

fn ask_editing_mode()->bool{//True = all | False = only(...)
    let mut w = String::new();
    w.push_str("Ingrese uno de los siguientes valores indicados\n");
    w.push_str("\t0 -> Para editar toda la carpeta\n");
    w.push_str("\t1 -> Para pasar a seleccionar los archivos a editar");
    loop {
        let edit = input_data_usize(&w);
        if edit == 0 {
            break true;
        }else if edit == 1{
            break false;
        }else{
            println!("Por favor intentelo de nuevo");
        }
    }
}
fn ask_quantity_a_edit()->usize{
    loop {
        let edit = input_data_usize("Ingrese la cantidad de archivos a editar");
        if edit > 0 {
            break edit;
        }else {
            println!("0 no es un numero valido");
        }
    }
}
type EditEntry = (String, String, EditFile);
fn ask_edit_file(olds:&Vec<String>, news:&Vec<String>, in_disk:&Vec<String>)->Vec<EditEntry>{
    let old = loop {
        let old = input_data("Ingrese el nombre del archivo que desea modificar");
        if olds.contains(&old){
            println!("Ya se selecciono este archivo para ser editado, ingrese otro")
        }else if news.contains(&old) {
            if in_disk.contains(&old) {
                break old;
            }else {
                println!("No puedo editar un archivo que no existe")
            }
        }else{
            if in_disk.contains(&old) {
                break old;
            }else {
                println!("No puedo editar un archivo que no existe")
            }
        }
    };
    ask_new_name_file(old, olds, news, in_disk, &mut Vec::new())
}
fn ask_new_name_file(old_name:String, olds:&Vec<String>, news:&Vec<String>, in_disk:&Vec<String>, edit:&mut Vec<EditEntry>)->Vec<EditEntry>{
    let mut name = String::from("Ingrese el nuevo nombre para ");
    name.push_str(&old_name);
    let (new, conflict) = loop {
        let new = input_data(&name);
        if news.contains(&new){
            println!("Ya hay un archivo preparado para ser editado con este nombre")
        }else if in_disk.contains(&new) {
            if olds.contains(&new){
                break (new, None);
            }else {
                break (new.clone(), Some(new));
            }
        }
    };

    edit.push(make_edit_file(&old_name, &new, conflict.is_none()));
    let mut olds = olds.to_vec();
    if !olds.contains(&old_name){olds.push(old_name);}
    let mut news = news.to_vec();
    if !news.contains(&new){news.push(new);}
    

    if conflict.is_some(){
        if let Some(old) = conflict{
            ask_new_name_file(old, &olds, &news, in_disk, edit)
        }else{
            edit.to_vec()
        }
    }else{
        edit.to_vec()
    }
}
fn make_edit_file(old_name:&String, new_name:&String, is_safe:bool)->EditEntry{
    let edit = EditFile::new_pending(old_name, new_name, is_safe);
    (old_name.into(), new_name.into(), edit)
}
