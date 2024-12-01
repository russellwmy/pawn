use anyhow::Result;
use wasmtime::*;

pub fn run_wasm_module(module_path: &str) -> Result<()> {
    // Create the Wasmtime engine and store
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // Load and compile the module
    let module = Module::from_file(&engine, module_path)?;
    let mut linker = Linker::new(&engine);

    // Define the `input` function
    linker.func_wrap("env", "input", |mut caller: Caller<'_, ()>, ptr: i32, len: i32| {
        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
        let data = memory.data_mut(&mut caller);

        // Example: Write user input into memory
        let input = "Hello, WebAssembly!\0"; // Ensure null-terminated
        let bytes = input.as_bytes();
        let write_len = bytes.len().min(len as usize);
        data[ptr as usize..ptr as usize + write_len].copy_from_slice(&bytes[..write_len]);
    })?;

    // Define the `console_log` function
    linker.func_wrap("env", "console_log", |mut caller: Caller<'_, ()>, ptr: i32| {
        let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
        let data = memory.data(&caller);

        // Read memory until null terminator
        if let Some(slice) = data.get(ptr as usize..) {
            if let Some(end) = slice.iter().position(|&c| c == 0) {
                println!("{}", String::from_utf8_lossy(&slice[..end]));
            } else {
                eprintln!("Error: Missing null terminator in memory");
            }
        } else {
            eprintln!("Error: Pointer out of bounds");
        }
    })?;

    // Instantiate the WebAssembly module
    let instance = linker.instantiate(&mut store, &module)?;

    // Call the `process_input` function
    let process_input = instance.get_func(&mut store, "process_input").unwrap();
    process_input.call(&mut store, &[], &mut [])?;

    Ok(())
}
