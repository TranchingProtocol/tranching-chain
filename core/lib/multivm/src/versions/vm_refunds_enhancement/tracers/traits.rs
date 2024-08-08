use micro_state::WriteStorage;

use crate::{
    interface::{
        dyn_tracers::vm_1_3_3::DynTracer,
        tracer::{TracerExecutionStatus, VmExecutionStopReason},
    },
    vm_refunds_enhancement::{
        bootloader_state::BootloaderState,
        old_vm::{history_recorder::HistoryMode, memory::SimpleMemory},
        types::internals::MicroVmState,
    },
};

pub type TracerPointer<S, H> = Box<dyn VmTracer<S, H>>;

/// Run tracer for collecting data during the vm execution cycles
pub trait VmTracer<S: WriteStorage, H: HistoryMode>: DynTracer<S, SimpleMemory<H>> {
    /// Initialize the tracer before the vm execution
    fn initialize_tracer(&mut self, _state: &mut MicroVmState<S, H>) {}
    /// Run after each vm execution cycle
    fn finish_cycle(
        &mut self,
        _state: &mut MicroVmState<S, H>,
        _bootloader_state: &mut BootloaderState,
    ) -> TracerExecutionStatus {
        TracerExecutionStatus::Continue
    }
    /// Run after the vm execution
    fn after_vm_execution(
        &mut self,
        _state: &mut MicroVmState<S, H>,
        _bootloader_state: &BootloaderState,
        _stop_reason: VmExecutionStopReason,
    ) {
    }
}

pub trait ToTracerPointer<S, H> {
    fn into_tracer_pointer(self) -> TracerPointer<S, H>;
}

impl<S: WriteStorage, H: HistoryMode, T: VmTracer<S, H> + 'static> ToTracerPointer<S, H> for T {
    fn into_tracer_pointer(self) -> TracerPointer<S, H> {
        Box::new(self)
    }
}
