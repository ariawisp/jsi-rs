#[cxx::bridge]
pub(crate) mod ffi {
    // Reuse base types to match JSI-SYS module structure
    #[namespace = "facebook::jsi"]
    unsafe extern "C++" {
        include!("jsi/jsi.h");

        // Aliases for base types
        #[cxx_name = "Object"]
        pub type JsiObject = crate::ffi::base::JsiObject;
        pub type Runtime = crate::ffi::base::Runtime;
        pub type NativeState;
    }

    #[namespace = "jsi_rs::ffi"]
    unsafe extern "C++" {
        include!("native_state.h");

        fn create_native_state_wrapper(rust_state: Box<RustNativeState>) -> SharedPtr<NativeState>;
        fn extract_rust_native_state(state: &SharedPtr<NativeState>) -> *mut RustNativeState;

        fn Object_hasNativeState(obj: &JsiObject, rt: Pin<&mut Runtime>) -> bool;
        fn Object_getNativeState(obj: &JsiObject, rt: Pin<&mut Runtime>) -> SharedPtr<NativeState>;
        fn Object_setNativeState(
            obj: Pin<&mut JsiObject>,
            rt: Pin<&mut Runtime>,
            state: SharedPtr<NativeState>,
        );
    }

    // Rust-side opaque type and helpers
    #[namespace = "jsi_rs::ffi"]
    extern "Rust" {
        type RustNativeState;
    }
}

pub use ffi::*;

use std::any::{Any, TypeId};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

// Container for arbitrary Send+Sync+'static state (type-erased via Any)
pub struct RustNativeState {
    inner: Arc<dyn Any + Send + Sync>,
}

impl RustNativeState {
    pub fn new_any(inner: Arc<dyn Any + Send + Sync>) -> Box<Self> {
        Box::new(Self { inner })
    }

    pub fn from_value<T: 'static + Send + Sync>(value: T) -> Box<Self> {
        Self::new_any(Arc::new(value))
    }

    pub fn from_arc<T: 'static + Send + Sync>(value: Arc<T>) -> Box<Self> {
        Self::new_any(value)
    }

    pub fn type_id(&self) -> TypeId {
        self.inner.type_id()
    }

    pub fn type_id_hash(&self) -> u64 {
        let tid = self.type_id();
        let mut hasher = DefaultHasher::new();
        tid.hash(&mut hasher);
        hasher.finish()
    }

    pub fn downcast_arc<T: 'static + Send + Sync>(&self) -> Option<Arc<T>> {
        // Try to downcast the erased Arc directly to Arc<T>
        let arc_any = self.inner.clone();
        if let Ok(v) = Arc::downcast::<T>(arc_any) {
            return Some(v);
        }

        // Fallback: the erased Arc may itself contain an Arc<T> (e.g. state set from an Arc)
        let arc_any = self.inner.clone();
        if let Ok(v) = Arc::downcast::<Arc<T>>(arc_any) {
            // Clone the inner Arc<T> to return ownership
            return Some(Arc::as_ref(&v).clone());
        }

        None
    }
}

// (no extern Rust functions required for C++; Rust crate uses inherent methods)
