# Olá Mundo!

[Ver documentação para este exemplo online](https://github.com/rustwasm/wasm-bindgen/tree/main/examples/without-a-bundler) 

Você pode construir o exemplo localmente com:

```
rm -rf node_modules  # Executar apenas uma vez
npm install          # Executar apenas uma vez
rm -rf pkg 
wasm-pack build --target web
python3 -m http.server
```

e, em seguida, visitar http://localhost:8000 em um navegador compativel para executar o exemplo!

Se precisar apenas compilar o código Rust para verificar erros, use o comando abaixo:

```shell
wasm-pack build --target web
```

## Referências

https://github.com/rustwasm

https://docs.rs/wasm-bindgen/0.2.75/wasm_bindgen/struct.JsValue.html

https://developer.mozilla.org/en-US/docs/Web/API/Window

https://rustwasm.github.io/wasm-bindgen/api/js_sys/

