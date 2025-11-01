use crate::{FromValue, JsiArray, JsiFn, JsiObject, JsiString, JsiValue, PropName, RuntimeHandle};

/// Convenience helpers for working with [`JsiObject`] instances.
pub trait JsiObjectExt<'rt> {
    /// Returns the property value as a [`JsiValue`] if it exists.
    fn get_value(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiValue<'rt>>;

    /// Returns the property as a Rust `String`.
    fn get_string(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<String>;

    /// Returns the property as a `bool`.
    fn get_bool(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<bool>;

    /// Returns the property as an `f64`.
    fn get_number(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<f64>;

    /// Returns the property as another [`JsiObject`].
    fn get_object(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiObject<'rt>>;

    /// Returns the property as a [`JsiArray`].
    fn get_array(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiArray<'rt>>;

    /// Returns the property as a [`JsiFn`].
    fn get_function(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiFn<'rt>>;
}

impl<'rt> JsiObjectExt<'rt> for JsiObject<'rt> {
    fn get_value(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiValue<'rt>> {
        if !self.has(PropName::new(key, rt), rt) {
            return None;
        }
        Some(self.get(PropName::new(key, rt), rt))
    }

    fn get_string(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<String> {
        self.get_value(key, rt)
            .and_then(|value| value.try_into_js::<JsiString>(rt))
            .map(|s| rt.to_string(&s))
    }

    fn get_bool(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<bool> {
        self.get_value(key, rt)
            .and_then(|value| FromValue::from_value(&value, rt))
    }

    fn get_number(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<f64> {
        self.get_value(key, rt)
            .and_then(|value| FromValue::from_value(&value, rt))
    }

    fn get_object(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiObject<'rt>> {
        self.get_value(key, rt)
            .and_then(|value| value.try_into_js::<JsiObject>(rt))
    }

    fn get_array(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiArray<'rt>> {
        self.get_value(key, rt)
            .and_then(|value| value.try_into_js::<JsiArray>(rt))
    }

    fn get_function(&self, key: &str, rt: &mut RuntimeHandle<'rt>) -> Option<JsiFn<'rt>> {
        self.get_value(key, rt)
            .and_then(|value| value.try_into_js::<JsiFn>(rt))
    }
}

/// Helpers for extracting native values from a [`JsiValue`].
pub trait JsiValueExt<'rt> {
    fn as_string(&self, rt: &mut RuntimeHandle<'rt>) -> Option<String>;
    fn as_bool(&self, rt: &mut RuntimeHandle<'rt>) -> Option<bool>;
    fn as_number(&self, rt: &mut RuntimeHandle<'rt>) -> Option<f64>;
    fn as_object(&self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiObject<'rt>>;
    fn as_array(&self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiArray<'rt>>;
    fn as_function(&self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiFn<'rt>>;
}

impl<'rt> JsiValueExt<'rt> for JsiValue<'rt> {
    fn as_string(&self, rt: &mut RuntimeHandle<'rt>) -> Option<String> {
        self.try_into_js::<JsiString>(rt).map(|s| rt.to_string(&s))
    }

    fn as_bool(&self, rt: &mut RuntimeHandle<'rt>) -> Option<bool> {
        FromValue::from_value(self, rt)
    }

    fn as_number(&self, rt: &mut RuntimeHandle<'rt>) -> Option<f64> {
        FromValue::from_value(self, rt)
    }

    fn as_object(&self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiObject<'rt>> {
        self.try_into_js::<JsiObject>(rt)
    }

    fn as_array(&self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiArray<'rt>> {
        self.try_into_js::<JsiArray>(rt)
    }

    fn as_function(&self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiFn<'rt>> {
        self.try_into_js::<JsiFn>(rt)
    }
}
