use wasmtime::*;

pub struct Runtime {
    pub engine: Engine,
    pub store: Store<()>,
    pub linker: Linker<()>,
}

impl Runtime {
    pub fn new() -> Self {
        let engine = Engine::default();
        let store = Store::new(&engine, ());
        let linker = Linker::new(&engine);

        Self { engine, store, linker }
    }
    pub fn load_module_from_bytes(
        &mut self,
        wasm_bytes: Vec<u8>,
    ) -> Result<Instance, Box<dyn std::error::Error>> {
        let module = Module::new(&self.engine, wasm_bytes)?;
        let instance = self.linker.instantiate(&mut self.store, &module)?;
        Ok(instance)
    }

    pub fn run_function<T, U>(
        &mut self,
        instance: &Instance,
        func_name: &str,
        args: T,
    ) -> Result<U, Box<dyn std::error::Error>>
    where
        T: WasmParams,
        U: WasmResults,
    {
        let func = instance.get_typed_func::<T, U>(&mut self.store, func_name)?;
        let result = func.call(&mut self.store, args)?;
        Ok(result)
    }
}
