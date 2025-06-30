// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

let engine

const canvas = document.getElementById('wasm-example')

canvas.setAttribute('tabindex','0');
canvas.focus();
let newSW = null;

// addEventListener("resize", (event) => { onresize(event) });

onresize = (event) => {
  if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
    if (engine) {
      engine.log("debug", "MsgFromJs", `canvas ${canvas.width} ${canvas.height}`)
    }
  }
}

rust
  .then(p => {
    engine = p.get_engine()
    engine.run()

    engine.log("Trace", "js", "cool")
    // p.console_log("wasm", "cool")
    // p.greet()
    // engine = p.init()
    // engine.run()
    // console.log(engine)
    // engine.start()
  })
  .catch(console.error)
  .catch((error) => {
    if (!error.message.startsWith("Using exceptions for control flow,")) {
      throw error;
    }
  })


// rust
//   .then(m => m.start())
//   .catch(console.error)
//   .catch((error) => {
//     if (!error.message.startsWith("Using exceptions for control flow,")) {
//       throw error;
//     }
//   })

// rust
//   .then(m => m.greet('World!'))
//   .catch(console.error);

// it not working, heh ----v
// import { greet } from './pkg'
// greet("Lol")