//! jsi-tm-macros: lightweight macros for writing Rust TurboModules.
//!
//! These macros build on top of the high-level `jsi` crate APIs and reduce
//! boilerplate when constructing JavaScript values and objects.
//!
//! Usage:
//! - Add `jsi` as a dependency in your crate.
//! - Add `jsi-tm-macros` as a dependency and import the macros.
//!
//! Example:
//! ```ignore
//! use jsi::{JsiValue, RuntimeHandle};
//! use jsi_tm_macros::{jsi_obj, jsi_arr};
//!
//! fn constants(rt: &mut RuntimeHandle) -> ::jsi::JsiObject {
//!     let dims = jsi_obj!(rt,
//!         "width" => 640.0,
//!         "height" => 360.0,
//!         "scale" => 1.0,
//!         "fontScale" => 1.0,
//!     );
//!     jsi_obj!(rt,
//!         "Dimensions" => dims,
//!         "flags" => jsi_arr!(rt, true, false, 1.0),
//!     )
//! }
//! ```

/// Build a JavaScript Object using the high-level `jsi` APIs.
///
/// - First argument is a mutable reference to a `RuntimeHandle` (e.g. `&mut rt`).
/// - Remaining arguments are `"key" => value` pairs.
/// - Values can be primitives (bool, f64, &str, String) or JSI wrappers
///   (JsiObject, JsiArray, JsiString, etc.) — anything implementing
///   `::jsi::IntoValue`.
#[macro_export]
macro_rules! jsi_obj {
    ($rt:expr, $( $key:expr => $val:expr ),* $(,)?) => {{
        let mut __obj = ::jsi::JsiObject::new($rt);
        $(
            let __val = ::jsi::IntoValue::into_value($val, $rt);
            __obj.set(::jsi::PropName::new($key, $rt), &__val, $rt);
        )*
        __obj
    }};
}

/// Build a JavaScript Array using the high-level `jsi` APIs.
///
/// - First argument is a mutable reference to a `RuntimeHandle`.
/// - Remaining arguments are array values (see `jsi_obj!` for accepted types).
#[macro_export]
macro_rules! jsi_arr {
    ($rt:expr, $( $val:expr ),* $(,)?) => {{
        // Count elements at macro-expansion time
        let __len = 0usize $(+ { let _ = &$val; 1usize })*;
        let mut __arr = ::jsi::JsiArray::new(__len, $rt);
        let mut __i = 0usize;
        $(
            let __val = ::jsi::IntoValue::into_value($val, $rt);
            __arr.set(__i, &__val, $rt);
            __i += 1;
        )*
        __arr
    }};
}

/// Set a property on a JavaScript Object using high-level `jsi` APIs.
///
/// - `$obj` must be a mutable `JsiObject` expression.
/// - `$rt` is a mutable reference to `RuntimeHandle`.
/// - `$key` is a property name (`&str` or string literal).
/// - `$val` implements `::jsi::IntoValue`.
#[macro_export]
macro_rules! jsi_set {
    ($obj:expr, $rt:expr, $key:expr, $val:expr) => {{
        let __val = ::jsi::IntoValue::into_value($val, $rt);
        $obj.set(::jsi::PropName::new($key, $rt), &__val, $rt);
    }};
}
