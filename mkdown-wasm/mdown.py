
import wasmtime
import markdown as md


# print(md.render('# Hello Python'))
# Import the custom loader for `*.wasm` files
import wasmtime.loader as ldr

# Assuming `your_wasm_file.wasm` is in the python load path...
# import pkg.mdown

# Now you're compiled and instantiated and ready to go!
# print(your_wasm_file.run())

from wasmtime import Store, Module, Instance, Func, FuncType

store = Store()
module = Module(store.engine, """
  (module
    (func $hello (import "" "hello"))
    (func (export "run") (call $hello))
  )
""")

def say_hello():
    print("Hello from Python!")

hello = Func(store, FuncType([], []), say_hello)

instance = Instance(store, module, [hello])
run = instance.exports(store)["run"]
run(store)
