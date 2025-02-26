To set up and run the backend, follow these steps:

1. **Install the WebAssembly (WASM) compiler:**

   ```sh
   rustup target add wasm32-wasip1
   ```

2. **Install the latest Wasmtime engine:**

   ```sh
   curl https://wasmtime.dev/install.sh -sSf | bash
   ```

3. **Add your function:**

   Place the function you want to execute into the `savedWasmFunction` folder.

4. **Build and start the server:**

   ```sh
   cargo run
   ```

   After running this command, visit [http://127.0.0.1:8080/](http://127.0.0.1:8080/) to verify that the server is working.

5. **Test serverless execution:**

   A sample WebAssembly module (`sample.wasm`) is available in the `savedWasmFunction` folder. You can test it by running:

   ```sh
   curl -X POST "http://localhost:8080/execute" -H "Content-Type: text/plain" -d "sample.wasm"
   ```

Thank you! 
