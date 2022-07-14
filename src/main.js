import "./app.css";
import App from "./App.svelte";
import init, { greet, airing, trending, build_trending } from '../wasm/pkg/wasm.js';
// Don't worry if vscode told you can't find my-crate
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install my-crate for you

init().then(() => {
  console.log('Hi Rust!');
  greet();
  const app = new App({
    target: document.body,
    props: {
      bindings: {
        airing,
        trending,
        build_trending
      }
    }
  });
});

// import wasm from '../rust/Cargo.toml';

// const init = async () => {
//   const bindings = await wasm();
//   const app = new App({
//     target: document.body,
//     props: {
//       bindings
//     },
//   });
// };
// init();

// const app = new App({
//   target: document.getElementById("app"),
// });

// export default app;
