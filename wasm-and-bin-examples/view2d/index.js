// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

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
