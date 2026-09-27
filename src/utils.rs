use std::io::{Write, stdin, stdout};

pub fn input_data(request:&str)->String{
    let mut buf = String::new();

    loop {
        buf.clear();
        println!("{}", request);
        print!("> ");
        stdout().flush().expect("No se pudo escribir en stdout");
        
        stdin().read_line(&mut buf).expect("Error de lectura");

        if buf.trim().is_empty() {
            println!("Intentalo de nuevo");
        }else {
            break;
        }
    }

    buf.trim().into()
}
pub fn input_data_usize(request:impl Into<String>)->usize{
    let request = request.into();
    loop {
        let count = input_data(&request);
        if !count.is_empty() {
            match count.parse(){
                Ok(u) => {break u;},
                Err(_) => println!("Ingresa solo numeros positivos y sin espacios")
            }
        }else {
            println!("No puede estar vacío");
        }
    }
}
pub fn work_to_fs_erros<W>(result:Result<W, std::io::Error>)->Option<W>{
    match result {
        Ok(w)=>Some(w),
        Err(e)=>{
            println!("Tipo de error: {}", e.kind());
            println!("raw: {:?}", e.raw_os_error());
            println!("\n\nERROR: {:?}\n", e);
            None
        }
    }
}