# Olá Mundo!

[Ver documentação para este exemplo online][dox] ou [Ver exemplo compilado
online][compilado]

[compilado]: https://rustwasm.github.io/wasm-bindgen/exbuild/hello_world/
[dox]: https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html

Você pode construir o exemplo localmente com:

```
rm -rf node_modules  # Executar apenas uma vez
npm install          # Executar apenas uma vez
rm -rf pkg 
npm run serve
```

e, em seguida, visitar http://localhost:8080 em um navegador compativel para executar o exemplo!

Se precisar apenas compilar o código Rust para verificar erros, use o comando abaixo:

```shell
cargo build --lib --target wasm32-unknown-unknown
```

## Referências

https://github.com/rustwasm

https://docs.rs/wasm-bindgen/0.2.75/wasm_bindgen/struct.JsValue.html

https://developer.mozilla.org/en-US/docs/Web/API/Window

https://rustwasm.github.io/wasm-bindgen/api/js_sys/

https://webpack.js.org/guides/asset-modules/#custom-output-filename 
