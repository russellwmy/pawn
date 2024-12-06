use wasmtime::component::ResourceTable;
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

/// Represents the state used by the WebAssembly component, including a resource table and a WASI context.
///
/// This struct encapsulates the resources and environment needed for the execution of WebAssembly modules
/// with WASI support.
pub struct State {
    http: WasiHttpCtx,

    /// The resource table which stores the resources (e.g., memory, tables) for the WebAssembly component.
    table: ResourceTable,

    /// The WASI context which provides access to the WASI environment.
    ctx: WasiCtx,
}

impl State {
    /// Creates a new `State` instance with a default resource table and WASI context.
    ///
    /// # Returns
    ///
    /// A new `State` instance with the default resources and context.
    pub fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().build();
        let http = WasiHttpCtx::new();
        Self { table, ctx, http }
    }
}

impl IoView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for State {
    /// Returns a mutable reference to the WASI context.
    ///
    /// This function is part of the `WasiView` trait and provides access to the WASI context, which contains
    /// the environment needed for running WASI-based WebAssembly modules.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `WasiCtx` associated with the state.
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for State {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
}
