// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const my_rust_code = import('./pkg');

// import Image from './ballista-architecture.png';
// import ballistaImg from './images/ballista-architecture.png'

let addToBody = function (str) {
    body = document.getElementsByTagName('body')[0];
    body.innerHTML += str
}

my_rust_code
  .then(
      m => addToBody(
        m.render_markdown('# Alô amigo Paraná!\nEste é um **teste** com imagem PNG' +
                '<br>![imagem](images/ballista-architecture.png)')
      )
   )
  .catch(console.error);

my_rust_code
  .then(m => console.log(
          m.render_markdown('# Alô Paraná!\nEste é um teste de mensagem vindo do Rust e escrita na console'))
      )
  .catch(console.error);

my_rust_code
  .then(m => m.greet('Paraná!'))
  .catch(console.error);
