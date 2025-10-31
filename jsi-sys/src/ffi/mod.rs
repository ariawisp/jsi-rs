#[cfg(target_os = "android")]
mod android;
pub mod base;
pub mod host;

#[cfg(target_os = "android")]
pub use android::*;
// Re-export base types (JSI core)
pub use base::*;
// Re-export host types (CxxHostObject and related functions)
// Note: RustHostObject, rho_* functions are pub(crate) and used by the FFI bridge
pub use host::{CxxHostObject, CxxHostObject_create, CxxHostObject_fromHostObjectS, CxxHostObject_fromHostObjectU,
              CxxHostObject_getInner, CxxHostObject_getInnerMut, CxxHostObject_toHostObjectS, CxxHostObject_toHostObjectU};
