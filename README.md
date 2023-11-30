# Namada SDK Tutorial/Template Project

This example project is based on the [namada-interface](https://github.com/anoma/namada-interface/tree/main) repo, but simplified as much as possible to give a "hello world" React app that uses the Namada sdk to query the current epoch.  

For a step-by-step of how to set this project up from scratch, and how the different parts interact, check out the tutorial [Part 1](https://hackmd.io/rgf-magIRUqyW9ObjQp3Ww) and [Part 2](https://hackmd.io/rgf-magIRUqyW9ObjQp3Ww).  

### Demo
From the project root, run
```
npm install
npm start
```
to start the app on `localhost:4000` (change the port by editing `app/.env`).

### Extending

You can add additional sdk functionality by adding to the Rust code in `namada/shared/lib/src/query.rs` and compiling to wasm:  
```
# (if not already done)
npm install

cd namada/shared

# (if not already installed)
cargo install -f wasm-bindgen-cli

npm run wasm:build:dev
# or npm run wasm:build:release
```



Then you can use your new method inside your React app by first calling the wasm `init` function and then creating a new instance of the Query class:
```
import Query from "@namada/shared";
import { init as initShared } from "@namada/shared/src/init";

await initShared()
const query = new Query("https://rpc.luminara.icu:443")
const epoch = await query.query_epoch()
console.log(epoch)
```