// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

const canvas = document.getElementById('wasm-example')

canvas.setAttribute('tabindex','0');
canvas.focus();

addEventListener("resize", (event) => {});

onresize = (event) => {
  if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
    console.log(canvas.width, canvas.height)
    // engine.resize(canvas.width, canvas.height)
  }
};

rust
  .then(m => m.start())
  .catch(console.error)
  .catch((error) => {
    if (!error.message.startsWith("Using exceptions for control flow,")) {
      throw error;
    }
  })

// rust
//   .then(m => m.greet('World!'))
//   .catch(console.error);

// it not working, heh ----v
// import { greet } from './pkg'
// greet("Lol")
