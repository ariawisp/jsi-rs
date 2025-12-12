#[cfg(target_os = "android")]
mod android;
pub mod base;
pub mod host;
pub mod native_state;

#[cfg(target_os = "android")]
pub use android::*;
// Re-export base types (JSI core)
pub use base::*;
// Re-export host types (CxxHostObject and related functions)
// Note: RustHostObject, rho_* functions are pub(crate) and used by the FFI bridge
pub use host::{
    CxxHostObject, CxxHostObject_create, CxxHostObject_fromHostObjectS,
    CxxHostObject_fromHostObjectU, CxxHostObject_getInner, CxxHostObject_getInnerMut,
    CxxHostObject_toHostObjectS, CxxHostObject_toHostObjectU,
};
// Re-export native state helpers
pub use native_state::{
    NativeState, Object_getNativeState, Object_hasNativeState, Object_setNativeState,
    RustNativeState, create_native_state_wrapper, extract_rust_native_state,
};
