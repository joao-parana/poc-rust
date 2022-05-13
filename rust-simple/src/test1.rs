#[wasmtime_rust::wasmtime]
trait WasmMarkdown  {
    fn render(&mut self, input: &str) -> String;
}

fn main() -> Result<(), failure::Error> {
    let mut markdown = WasmMarkdown::load_file("/opt/DEV/poc-rust/mkdown-wasm/pkg/mdown.wasm")?;
    println!("{}", makrdown.render("# Hello Rust!"));
    Ok(())
}
