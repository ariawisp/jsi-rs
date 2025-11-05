use cxx::UniquePtr;
use jsi::RuntimeHandle;

use crate::ffi::bridge::*;

pub struct Harness {
    raw: UniquePtr<jsi_sys::Runtime>,
}

impl Harness {
    pub fn new() -> Self {
        let config = create_runtime_config();
        let rt = create_hermes_runtime(&*config);
        let raw = cast_hermes_runtime(rt);
        Harness { raw }
    }

    pub fn run<R>(&mut self, f: impl for<'rt> FnOnce(&'rt mut RuntimeHandle<'rt>) -> R) -> R {
        // Safety: the Hermes runtime lives for the duration of self.raw.
        let mut handle = unsafe { RuntimeHandle::from_raw_pin(self.raw.pin_mut()) };
        let out = f(&mut handle);
        // Clear any cached PropNameIDs that would dangle after the runtime drops.
        jsi::cached_prop_name::clear_cache();
        out
    }
}
