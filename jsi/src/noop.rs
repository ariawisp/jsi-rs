use cxx::CxxString;
use std::pin::Pin;

/// Shared no-op implementation for TurboModule listener bindings.
#[inline]
pub unsafe fn noop_add_listener<T>(_rt: Pin<&mut T>, _event_type: &CxxString) {}

/// Shared no-op implementation for TurboModule listener bindings.
#[inline]
pub unsafe fn noop_remove_listeners<T>(_rt: Pin<&mut T>, _count: f64) {}
