use std::fs::File;

fn main() {
    let var: u8 = 128;
    println!("Olá mundo ! var = {}", var); 
    println!("var = {}, tamanho = {} bytes", var, std::mem::size_of_val(&var));

    let f = File::open("main.rs");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error)
        },
    };

}

