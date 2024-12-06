//! A module for interacting with WebAssembly (Wasm) components using the `wasmtime` runtime.
//!
//! This module provides a `Component` struct that encapsulates a Wasm component, its linker, and store,
//! along with methods to instantiate and call functions within the component.

// use alloc::string::{String, ToString};

use derive_builder::Builder;
use snafu::{prelude::*, ResultExt};
use wasmtime::{
    component::{
        Component as WasmComponent, ComponentType, Func, Instance, Linker, TypedFunc, Val,
    },
    Engine, Store,
};

use crate::{state::State, Runtime};

/// Enum to represent errors that can occur when working with Wasm components.
#[derive(Debug, Snafu)]
pub enum ComponentError {
    #[snafu(display("Failed to create Wasm component: {}", source))]
    WasmComponentCreationFailed { source: wasmtime::Error },

    #[snafu(display("Failed to instantiate Wasm component: {}", source))]
    WasmComponentInstantiateFailed { source: wasmtime::Error },

    #[snafu(display("Failed to link WASI: {}", source))]
    WasiLinkingFailed { source: wasmtime::Error },

    #[snafu(display("Failed to read WASM file"))]
    ReadWasmFailed,

    #[snafu(display("Failed to set runtime"))]
    RuntimeSetFailed,

    #[snafu(display("Failed to initialize function: {} {}", name, source))]
    ComponentFunctionInitializeFailed { name: String, source: wasmtime::Error },

    #[snafu(display("Exported function '{}' not found", name))]
    FunctionExportNotFound { name: String },

    #[snafu(display("Failed to call function: {}", source))]
    ComponentFunctionCallFailed { source: wasmtime::Error },

    #[snafu(display("Exported module '{}' not found", name))]
    ModuleExportNotFound { name: String },

    #[snafu(display("Exported  Handler '{}' not found", name))]
    HandlerExportNotFound { name: String },
}

type Result<T, E = ComponentError> = core::result::Result<T, E>;

/// A struct representing a WebAssembly component.
///
/// This struct encapsulates the Wasm component, its linker, store, and runtime, providing methods
/// to interact with the component, such as calling exported functions.
#[derive(Builder)]
#[builder(no_std)]
#[builder(setter(into))]
#[builder(build_fn(skip))]
pub struct Component<'a> {
    /// The Wasm component instance created from the WASM binary.
    #[builder(setter(skip))]
    component: WasmComponent,

    /// The linker used to link the component with the WASI environment.
    #[builder(setter(skip))]
    linker: Linker<State>,

    /// The store in which the component and WASI environment are instantiated.
    #[builder(setter(skip))]
    store: Store<State>,

    /// The raw WASM binary.
    #[allow(dead_code)]
    wasm: &'a [u8],

    /// A reference to the runtime, which is needed for component instantiation.
    pub runtime: &'a Runtime,
}

impl<'a> ComponentBuilder<'a> {
    /// Builds the `Component` by configuring the runtime, WASM binary, and necessary dependencies such as the store
    /// and linker. If any of these are missing or incorrect, an error is returned.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the created `Component` instance or an error if any part of the process fails.
    pub fn build(&self) -> Result<Component<'a>> {
        let runtime = self.runtime.ok_or(ComponentError::RuntimeSetFailed)?;
        let wasm = self.wasm.ok_or(ComponentError::ReadWasmFailed)?;
        let engine = &runtime.engine;

        // Create a new store with the provided engine
        let state = State::new();
        let store = Store::new(engine, state);

        // Initialize the linker and add WASI support
        let mut linker = Linker::new(engine);
        wasmtime_wasi::add_to_linker_async(&mut linker).context(WasiLinkingFailedSnafu)?;
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)
            .context(WasiLinkingFailedSnafu)?;

        // Create the component from the WASM binary
        let component =
            WasmComponent::new(engine, wasm).context(WasmComponentCreationFailedSnafu)?;

        Ok(Component { component, linker, store, wasm, runtime })
    }
}

impl<'a> Component<'a> {
    /// Creates a new `Component` with the specified runtime and WASM binary.
    ///
    /// # Parameters
    /// - `wasm`: A reference to the WASM binary that will be used to create the component.
    /// - `runtime`: A reference to the runtime in which the component will be instantiated.
    ///
    /// # Returns
    ///
    /// A `Result` containing the created `Component` instance or an error if the creation fails.
    pub fn with_runtime(wasm: &'a [u8], runtime: &'a Runtime) -> Result<Component<'a>> {
        ComponentBuilder::default().wasm(wasm).runtime(runtime).build()
    }

    /// Looks up an exported function from the component by its name. This method allows calling
    /// a specific function within the WebAssembly component.
    ///
    /// # Parameters
    /// - `namespace`: Optional namespace to filter exported functions.
    /// - `function_name`: The name of the exported function to look up.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Func` if found, or an error if the function is not found.
    ///
    /// # Errors
    ///
    /// - `ComponentError::FunctionExportNotFound`: If the function is not found.
    /// - `ComponentError::WasmComponentInstantiateFailed`: If the component instantiation fails.
    pub async fn _component_export_function_lookup(
        &mut self,
        namespace: Option<&str>,
        function_name: &str,
    ) -> Result<Func> {
        let instance = self
            .linker
            .instantiate_async(&mut self.store, &self.component)
            .await
            .context(WasmComponentInstantiateFailedSnafu)?;
        for (export_name, ty) in self.component.component_type().exports(&self.runtime.engine) {
            if let Some(namespace) = namespace {
                if !export_name.starts_with(namespace) {
                    continue;
                }
            }
            match ty {
                wasmtime::component::types::ComponentItem::ComponentFunc(component_func) => {
                    let (_, handle_export_index) = self
                        .component
                        .export_index(None, export_name)
                        .ok_or(
                        ComponentError::FunctionExportNotFound { name: function_name.to_string() },
                    )?;

                    let func = instance.get_func(&mut self.store, handle_export_index).ok_or(
                        ComponentError::FunctionExportNotFound { name: function_name.to_string() },
                    )?;

                    return Ok(func);
                }
                wasmtime::component::types::ComponentItem::CoreFunc(func_type) => todo!(),
                wasmtime::component::types::ComponentItem::Module(module) => todo!(),
                wasmtime::component::types::ComponentItem::Component(component) => todo!(),
                wasmtime::component::types::ComponentItem::ComponentInstance(
                    component_instance,
                ) => {
                    let (_, handler_export_index) = self
                        .component
                        .export_index(None, export_name)
                        .ok_or(ComponentError::ModuleExportNotFound {
                            name: function_name.to_string(),
                        })?;
                    let (_, handle_export_index) = self
                        .component
                        .export_index(Some(&handler_export_index), "handle")
                        .ok_or(ComponentError::FunctionExportNotFound {
                            name: function_name.to_string(),
                        })?;
                    let func = instance.get_func(&mut self.store, handle_export_index).ok_or(
                        ComponentError::FunctionExportNotFound { name: function_name.to_string() },
                    )?;

                    return Ok(func);
                }
                wasmtime::component::types::ComponentItem::Type(_) => todo!(),
                wasmtime::component::types::ComponentItem::Resource(resource_type) => todo!(),
            }
        }

        Err(ComponentError::FunctionExportNotFound { name: function_name.to_string() })
    }

    /// Calls the specified function from the component with the given parameters.
    ///
    /// # Parameters
    /// - `namespace`: Optional namespace to filter exported functions.
    /// - `function_name`: The name of the function to call within the WebAssembly component.
    /// - `params`: The parameters to pass to the function when calling it.
    /// - `results`: Mutable slice to store function results.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    ///
    /// # Errors
    ///
    /// - `ComponentError::ComponentFunctionCallFailed`: If the function call fails.
    /// - `ComponentError::FunctionExportNotFound`: If the exported function is not found.
    /// - `ComponentError::WasmComponentInstantiateFailed`: if component instantiation
    pub async fn call(
        mut self,
        namespace: Option<&str>,
        function_name: &str,
        params: &[Val],
    ) -> Result<Vec<Val>> {
        let func = self._component_export_function_lookup(namespace, function_name).await?;
        let num_results = func.results(&self.store).len();
        let mut results = vec![Val::Option(None); num_results];

        // Call the function with the provided parameters
        func.call_async(&mut self.store, params, &mut results)
            .await
            .map_err(|e| ComponentError::ComponentFunctionCallFailed { source: e })?;
        Ok(results)
    }
}
