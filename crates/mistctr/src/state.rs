use wasmtime_wasi::{
    ResourceTable,
    p2::{IoView, WasiCtx, WasiView},
};

pub struct HostState {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
}

impl IoView for HostState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for HostState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
