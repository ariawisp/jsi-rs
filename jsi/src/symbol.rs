use std::marker::PhantomData;

use crate::{JsiFn, JsiValue, PropName, RuntimeDisplay, RuntimeEq, RuntimeHandle, sys};

/// A JavaScript `Symbol`
pub struct JsiSymbol<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiSymbol>,
    pub(crate) PhantomData<&'rt ()>,
);

impl RuntimeEq for JsiSymbol<'_> {
    fn eq(&self, other: &Self, rt: &mut RuntimeHandle<'_>) -> bool {
        sys::Symbol_compare(
            rt.get_inner_mut(),
            self.0.as_ref().unwrap(),
            other.0.as_ref().unwrap(),
        )
    }
}

impl RuntimeDisplay for JsiSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, rt: &mut RuntimeHandle<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string(rt.get_inner_mut()))
    }
}

unsafe impl<'rt> Send for JsiSymbol<'rt> {}

impl<'rt> JsiSymbol<'rt> {
    /// Create a new JavaScript `Symbol`. If a description is provided, uses
    /// `Symbol(description)`, otherwise uses `Symbol()`.
    pub fn new(description: Option<&str>, rt: &mut RuntimeHandle<'rt>) -> Self {
        // Obtain global Symbol function
        let sym_ctor_val = rt.global().get(PropName::new("Symbol", rt), rt);
        let sym_fn: JsiFn = sym_ctor_val
            .try_into_js(rt)
            .expect("global Symbol is not a function");

        let value = match description {
            Some(desc) => sym_fn
                .call(std::iter::once(JsiValue::new_string(desc, rt)), rt)
                .expect("Symbol(desc) invocation failed"),
            None => sym_fn
                .call(std::iter::empty(), rt)
                .expect("Symbol() invocation failed"),
        };

        value
            .try_into_js::<JsiSymbol>(rt)
            .expect("Symbol() did not return a symbol")
    }
}

// IntoValue for JsiSymbol is implemented in value.rs
