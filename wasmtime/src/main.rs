use anyhow;


fn main() -> anyhow::Result<()> {
    let mut markdown = WasmMarkdown::load_file("markdown.wasm")?;
    println!("{}", markdown.render("# Hello, Rust!"));

    Ok(())
}

#[wasmtime_rust::wasmtime]
trait WasmMarkdown {
    fn render(&mut self, input: &str) -> String;
}

