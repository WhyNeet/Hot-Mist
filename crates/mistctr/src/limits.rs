use wasmtime::{
    DEFAULT_INSTANCE_LIMIT, DEFAULT_MEMORY_LIMIT, DEFAULT_TABLE_LIMIT, ResourceLimiter,
    ResourceLimiterAsync,
};
use wasmtime_wasi::async_trait;

#[derive(Clone, Debug)]
pub struct StoreLimitsAsync {
    memory_size: Option<usize>,
    table_elements: Option<usize>,
    instances: usize,
    tables: usize,
    memories: usize,
    trap_on_grow_failure: bool,
}

impl Default for StoreLimitsAsync {
    fn default() -> Self {
        Self {
            memory_size: None,
            table_elements: None,
            instances: DEFAULT_INSTANCE_LIMIT,
            tables: DEFAULT_TABLE_LIMIT,
            memories: DEFAULT_MEMORY_LIMIT,
            trap_on_grow_failure: false,
        }
    }
}

impl ResourceLimiter for StoreLimitsAsync {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        let allow = match self.memory_size {
            Some(limit) if desired > limit => false,
            _ => match maximum {
                Some(max) if desired > max => false,
                _ => true,
            },
        };
        if !allow && self.trap_on_grow_failure {
            anyhow::bail!("forcing trap when growing memory to {desired} bytes")
        } else {
            Ok(allow)
        }
    }

    fn memory_grow_failed(&mut self, error: anyhow::Error) -> wasmtime::Result<()> {
        if self.trap_on_grow_failure {
            Err(error.context("forcing a memory growth failure to be a trap"))
        } else {
            tracing::debug!("ignoring memory growth failure error: {error:?}");
            Ok(())
        }
    }

    fn table_growing(
        &mut self,
        _current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        let allow = match self.table_elements {
            Some(limit) if desired > limit => false,
            _ => match maximum {
                Some(max) if desired > max => false,
                _ => true,
            },
        };
        if !allow && self.trap_on_grow_failure {
            anyhow::bail!("forcing trap when growing table to {desired} elements")
        } else {
            Ok(allow)
        }
    }

    fn table_grow_failed(&mut self, error: anyhow::Error) -> wasmtime::Result<()> {
        if self.trap_on_grow_failure {
            Err(error.context("forcing a table growth failure to be a trap"))
        } else {
            tracing::debug!("ignoring table growth failure error: {error:?}");
            Ok(())
        }
    }

    fn instances(&self) -> usize {
        self.instances
    }

    fn tables(&self) -> usize {
        self.tables
    }

    fn memories(&self) -> usize {
        self.memories
    }
}

#[async_trait]
impl ResourceLimiterAsync for StoreLimitsAsync {
    async fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        // Defer to sync logic (you could add async checks here if needed)
        <Self as ResourceLimiter>::memory_growing(self, current, desired, maximum)
    }

    async fn table_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        <Self as ResourceLimiter>::table_growing(self, current, desired, maximum)
    }
}

/// Used to build [`StoreLimits`].
pub struct StoreLimitsAsyncBuilder(StoreLimitsAsync);

impl StoreLimitsAsyncBuilder {
    pub fn new() -> Self {
        Self(StoreLimitsAsync::default())
    }

    /// The maximum number of bytes a linear memory can grow to.
    ///
    /// Growing a linear memory beyond this limit will fail. This limit is
    /// applied to each linear memory individually, so if a wasm module has
    /// multiple linear memories then they're all allowed to reach up to the
    /// `limit` specified.
    ///
    /// By default, linear memory will not be limited.
    pub fn memory_size(mut self, limit: usize) -> Self {
        self.0.memory_size = Some(limit);
        self
    }

    /// The maximum number of elements in a table.
    ///
    /// Growing a table beyond this limit will fail. This limit is applied to
    /// each table individually, so if a wasm module has multiple tables then
    /// they're all allowed to reach up to the `limit` specified.
    ///
    /// By default, table elements will not be limited.
    pub fn table_elements(mut self, limit: usize) -> Self {
        self.0.table_elements = Some(limit);
        self
    }

    /// The maximum number of instances that can be created for a [`Store`](crate::Store).
    ///
    /// Module instantiation will fail if this limit is exceeded.
    ///
    /// This value defaults to 10,000.
    pub fn instances(mut self, limit: usize) -> Self {
        self.0.instances = limit;
        self
    }

    /// The maximum number of tables that can be created for a [`Store`](crate::Store).
    ///
    /// Module instantiation will fail if this limit is exceeded.
    ///
    /// This value defaults to 10,000.
    pub fn tables(mut self, tables: usize) -> Self {
        self.0.tables = tables;
        self
    }

    /// The maximum number of linear memories that can be created for a [`Store`](crate::Store).
    ///
    /// Instantiation will fail with an error if this limit is exceeded.
    ///
    /// This value defaults to 10,000.
    pub fn memories(mut self, memories: usize) -> Self {
        self.0.memories = memories;
        self
    }

    /// Indicates that a trap should be raised whenever a growth operation
    /// would fail.
    ///
    /// This operation will force `memory.grow` and `table.grow` instructions
    /// to raise a trap on failure instead of returning -1. This is not
    /// necessarily spec-compliant, but it can be quite handy when debugging a
    /// module that fails to allocate memory and might behave oddly as a result.
    ///
    /// This value defaults to `false`.
    pub fn trap_on_grow_failure(mut self, trap: bool) -> Self {
        self.0.trap_on_grow_failure = trap;
        self
    }

    /// Consumes this builder and returns the [`StoreLimits`].
    pub fn build(self) -> StoreLimitsAsync {
        self.0
    }
}
