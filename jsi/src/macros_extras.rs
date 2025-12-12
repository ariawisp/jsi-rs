//! Additional helper macros for ergonomic JSI usage.

/// Build a JavaScript Object using the high-level `jsi` APIs.
/// First arg is `&mut RuntimeHandle`, followed by `"key" => value` pairs.
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
/// First arg is `&mut RuntimeHandle`, followed by array elements.
#[macro_export]
macro_rules! jsi_arr {
    ($rt:expr, $( $val:expr ),* $(,)?) => {{
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
#[macro_export]
macro_rules! jsi_set {
    ($obj:expr, $rt:expr, $key:expr, $val:expr) => {{
        let __val = ::jsi::IntoValue::into_value($val, $rt);
        $obj.set(::jsi::PropName::new($key, $rt), &__val, $rt);
    }};
}

/// Create a JavaScript Promise using `jsi::create_promise` (JSI-agnostic).
#[macro_export]
macro_rules! jsi_promise {
    ($rt:expr, |$resolve:ident, $reject:ident| $body:block) => {{ ::jsi::create_promise(|$resolve, $reject, _rt| $body, $rt) }};
    ($rt:expr, |$resolve:ident, $reject:ident, $hrt:ident| $body:block) => {{ ::jsi::create_promise(|$resolve, $reject, $hrt| $body, $rt) }};
}
