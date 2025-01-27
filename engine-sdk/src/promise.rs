use aurora_engine_types::parameters::{
    PromiseBatchAction, PromiseCreateArgs, PromiseWithCallbackArgs,
};
use aurora_engine_types::types::PromiseResult;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PromiseId(u64);

impl PromiseId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

pub trait PromiseHandler {
    type ReadOnly: ReadOnlyPromiseHandler;

    fn promise_results_count(&self) -> u64;
    fn promise_result(&self, index: u64) -> Option<PromiseResult>;

    fn promise_create_call(&mut self, args: &PromiseCreateArgs) -> PromiseId;
    fn promise_attach_callback(
        &mut self,
        base: PromiseId,
        callback: &PromiseCreateArgs,
    ) -> PromiseId;
    fn promise_create_batch(&mut self, args: &PromiseBatchAction) -> PromiseId;
    fn promise_return(&mut self, promise: PromiseId);

    fn promise_create_with_callback(&mut self, args: &PromiseWithCallbackArgs) -> PromiseId {
        let base = self.promise_create_call(&args.base);
        self.promise_attach_callback(base, &args.callback)
    }

    fn read_only(&self) -> Self::ReadOnly;
}

pub trait ReadOnlyPromiseHandler {
    fn ro_promise_results_count(&self) -> u64;
    fn ro_promise_result(&self, index: u64) -> Option<PromiseResult>;
}

impl<T: PromiseHandler> ReadOnlyPromiseHandler for T {
    fn ro_promise_results_count(&self) -> u64 {
        self.promise_results_count()
    }

    fn ro_promise_result(&self, index: u64) -> Option<PromiseResult> {
        self.promise_result(index)
    }
}

/// A promise handler which does nothing. Should only be used when promises can be safely ignored.
#[derive(Debug, Copy, Clone)]
pub struct Noop;

impl PromiseHandler for Noop {
    type ReadOnly = Self;

    fn promise_results_count(&self) -> u64 {
        0
    }

    fn promise_result(&self, _index: u64) -> Option<PromiseResult> {
        None
    }

    fn promise_create_call(&mut self, _args: &PromiseCreateArgs) -> PromiseId {
        PromiseId::new(0)
    }

    fn promise_attach_callback(
        &mut self,
        _base: PromiseId,
        _callback: &PromiseCreateArgs,
    ) -> PromiseId {
        PromiseId::new(0)
    }

    fn promise_create_batch(&mut self, _args: &PromiseBatchAction) -> PromiseId {
        PromiseId::new(0)
    }

    fn promise_return(&mut self, _promise: PromiseId) {}

    fn read_only(&self) -> Self::ReadOnly {
        Self
    }
}
