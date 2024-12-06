## Development  

To set up the development environment, run:  

```sh
.devcontainer/startup.sh
```

## Folder Structures

- [chat](./chat) - A WebAssembly component designed to process chat messages.
- [runtime](./runtime) - A simple WebAssembly components runtime.

## WebAssembly Components  

### Create a new WebAssembly component  

   ```sh
   cargo component new <component-name>
   ```

### Build the component in release mode  

   ```sh
   cargo component build --release
   ```

### Build chat component

   ```sh
   cd chat
   cargo component build --release
   ```
