fn main() {
    let var: u8 = 128;
    println!("Olá mundo ! var = {}", var);
    println!("var = {}, tamanho = {} bytes", var, std::mem::size_of_val(&var));
}
