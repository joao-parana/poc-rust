use std::fs::File;
// use anyhow;
// use failure;

// anyhow::Result<()>
// Result<(), failure::Error>
fn main() {
    let var: u8 = 128;
    println!("Olá mundo ! var = {}", var);
    println!("var = {}, tamanho = {} bytes", var, std::mem::size_of_val(&var));

    let f = File::open("/opt/DEV/poc-rust/rust-simple/src/main.rs");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}", error)
        },
    };
}
