# import wasmtime.loader
# import rust_wasi_markdown_parser as parser

# print("Markdown result = ", parser.main('# Hello Python'))
# print("Markdown result = ", parser.render_mkdn('# Hello Python'))

from wasmtime import Store, Module, Instance

store = Store()
module = Module.from_file(store.engine, 'rust_wasi_markdown_parser.wasm')
instance = Instance(store, module, [])
render_mkdn = instance.get_export('render_mkdn')
print("render_mkdn =", render_mkdn('# Hello Python'))



