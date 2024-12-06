use core::time::Duration;

use derive_builder::Builder;
use snafu::ResultExt;
use tracing::warn;
use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig};

/// Default maximum linear memory for a component (256 MiB)
pub const MAX_LINEAR_MEMORY: u64 = 256 * 1024 * 1024;
/// Default maximum component size (50 MiB)
pub const MAX_COMPONENT_SIZE: u64 = 50 * 1024 * 1024;
/// Default maximum number of components
pub const MAX_COMPONENTS: u32 = 10_000;

#[derive(Debug, snafu::Snafu)]
pub enum RuntimeError {
    #[snafu(display("Failed to create pooling allocator: {}", source))]
    PoolingAllocatorCreationFailed { source: wasmtime::Error },
    #[snafu(display("Failed to create dynamic allocator: {}", source))]
    DynamicAllocatorCreationFailed { source: wasmtime::Error },
}

type Result<T, E = RuntimeError> = core::result::Result<T, E>;

/// Represents the configuration and state for the runtime engine.
///
/// This struct contains the settings for managing Wasmtime components and memory allocations.
/// It also holds the engine and its configuration that drives the execution of components in the runtime.
#[derive(Default, Builder)]
#[builder(no_std)]
#[builder(setter(into))]
#[builder(build_fn(skip))]
pub struct Runtime {
    /// The Wasmtime engine that executes components.
    #[builder(setter(skip))]
    pub engine: Engine,

    /// The configuration for the Wasmtime engine.
    #[builder(setter(skip))]
    pub engine_config: Config,

    /// The maximum execution time for a component before it is considered to have failed.
    #[builder(default = "Duration::from_secs(10)")]
    pub max_execution_time: Duration,

    /// The maximum number of components the runtime will manage.
    #[allow(dead_code)]
    #[builder(default = "MAX_COMPONENTS")]
    max_components: u32,

    /// The maximum size for a component, in bytes.
    #[allow(dead_code)]
    #[builder(default = "MAX_COMPONENT_SIZE")]
    max_component_size: usize,

    /// The maximum size for the linear memory, in bytes.
    #[allow(dead_code)]
    #[builder(default = "MAX_LINEAR_MEMORY")]
    max_linear_memory: usize,

    /// Whether to force the use of a pooling allocator (may reduce performance but improves memory allocation).
    #[allow(dead_code)]
    #[builder(default = "false")]
    force_pooling_allocator: bool,

    /// The number of memories each component can have.
    #[allow(dead_code)]
    #[builder(default = "1")]
    memories_per_component: u32,

    /// The number of tables each component can have.
    #[allow(dead_code)]
    #[builder(default = "1")]
    tables_per_component: u32,

    /// The maximum number of core instances that each component can have.
    #[allow(dead_code)]
    #[builder(default = "30")]
    max_core_instances_per_component: u32,

    /// The number of elements in each table.
    #[allow(dead_code)]
    #[builder(default = "15_000")]
    table_elements: usize,

    /// The maximum number of tables a component can have.
    #[allow(dead_code)]
    #[builder(default = "20")]
    max_tables_per_component: u32,
}

impl RuntimeBuilder {
    /// Builds a `Runtime` instance from the provided configuration.
    ///
    /// This method configures memory allocation, sets up the Wasmtime engine, and creates the `Runtime` instance.
    /// It tries to use a pooling allocator first, but if that fails, it falls back to a dynamic allocator.
    ///
    /// # Returns
    ///
    /// A result containing either the created `Runtime` instance or an error if creation fails.
    fn build(&self) -> Result<Runtime> {
        // Extract builder fields with defaults
        let max_components = self.max_components.unwrap_or(MAX_COMPONENTS);
        let max_component_size = self.max_component_size.unwrap_or(MAX_COMPONENT_SIZE as usize);
        let max_linear_memory = self.max_linear_memory.unwrap_or(MAX_LINEAR_MEMORY as usize);
        let force_pooling_allocator = self.force_pooling_allocator.unwrap_or(false);
        let memories_per_component = self.memories_per_component.unwrap_or(1);
        let tables_per_component = self.tables_per_component.unwrap_or(1);
        let max_core_instances_per_component = self.max_core_instances_per_component.unwrap_or(30);
        let table_elements = self.table_elements.unwrap_or(15_000);
        let max_tables_per_component = self.max_tables_per_component.unwrap_or(20);

        // Configure pooling allocation
        let mut pooling_config = PoolingAllocationConfig::default();

        pooling_config
            .total_component_instances(max_components)
            .total_core_instances(max_components)
            .total_stacks(max_components)
            .max_component_instance_size(max_component_size)
            .max_core_instances_per_component(max_core_instances_per_component)
            .max_tables_per_component(max_tables_per_component)
            .table_elements(table_elements)
            .max_memories_per_component(max_core_instances_per_component * memories_per_component)
            .total_memories(max_components * memories_per_component)
            .total_tables(max_components * tables_per_component)
            .max_memory_size(max_linear_memory);

        let mut engine_config = Config::new();
        engine_config
            .async_support(true)
            .wasm_component_model(true)
            .allocation_strategy(InstanceAllocationStrategy::Pooling(pooling_config));

        // Create the engine
        let engine = match Engine::new(&engine_config) {
            Ok(engine) => engine,
            Err(e) if force_pooling_allocator => {
                return Err(RuntimeError::PoolingAllocatorCreationFailed { source: e });
            }
            Err(e) => {
                warn!(
                    "Falling back to dynamic allocator due to error: {}. This may reduce performance.",
                    e
                );
                let mut fallback_config = engine_config.clone();
                fallback_config.allocation_strategy(InstanceAllocationStrategy::OnDemand);
                Engine::new(&fallback_config).context(DynamicAllocatorCreationFailedSnafu)?
            }
        };

        Ok(Runtime {
            engine,
            engine_config,
            max_execution_time: self.max_execution_time.unwrap_or(Duration::from_secs(10)),
            max_components,
            max_component_size,
            max_linear_memory,
            force_pooling_allocator,
            memories_per_component,
            tables_per_component,
            max_core_instances_per_component,
            table_elements,
            max_tables_per_component,
        })
    }
}

impl Runtime {
    /// Creates a new `Runtime` instance with default settings.
    ///
    /// This is a helper method that invokes the default builder to create the `Runtime`.
    ///
    /// # Returns
    ///
    /// A result containing the created `Runtime` or an error if the creation fails.
    pub fn new() -> Result<Self> {
        RuntimeBuilder::default().build()
    }
}
