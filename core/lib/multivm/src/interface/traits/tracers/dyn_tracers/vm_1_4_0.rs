use micro_state::StoragePtr;
use zk_evm_1_4_0::{
    abstractions::Memory,
    tracing::{AfterDecodingData, AfterExecutionData, BeforeExecutionData, VmLocalStateData},
};

/// Version of zk_evm_1_3_3::Tracer suitable for dynamic dispatch.
pub trait DynTracer<S, M: Memory> {
    fn before_decoding(&mut self, _state: VmLocalStateData<'_>, _memory: &M) {}
    fn after_decoding(
        &mut self,
        _state: VmLocalStateData<'_>,
        _data: AfterDecodingData,
        _memory: &M,
    ) {
    }
    fn before_execution(
        &mut self,
        _state: VmLocalStateData<'_>,
        _data: BeforeExecutionData,
        _memory: &M,
        _storage: StoragePtr<S>,
    ) {
    }
    fn after_execution(
        &mut self,
        _state: VmLocalStateData<'_>,
        _data: AfterExecutionData,
        _memory: &M,
        _storage: StoragePtr<S>,
    ) {
    }
}
