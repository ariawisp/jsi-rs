use std::marker::PhantomData;

use crate::{JsiObject, JsiValue, RuntimeHandle, sys};

/// A JavaScript `WeakObject` wrapper allowing weak references to objects.
#[cfg_attr(docsrs, doc(cfg(feature = "weak_object")))]
pub struct JsiWeakObject<'rt>(
    pub(crate) cxx::UniquePtr<sys::JsiWeakObject>,
    pub(crate) PhantomData<&'rt mut ()>,
);

impl<'rt> JsiWeakObject<'rt> {
    pub fn new(obj: &JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>) -> Self {
        let weak_ptr = sys::WeakObject_fromObject(rt.get_inner_mut(), obj.0.as_ref().unwrap());
        Self(weak_ptr, PhantomData)
    }

    pub fn lock(&mut self, rt: &mut RuntimeHandle<'rt>) -> Option<JsiValue<'rt>> {
        let value_ptr = sys::WeakObject_lock(self.0.pin_mut(), rt.get_inner_mut());
        let value = JsiValue(value_ptr, PhantomData);
        if value.is_undefined() {
            None
        } else {
            Some(value)
        }
    }
}

unsafe impl<'rt> Send for JsiWeakObject<'rt> {}
