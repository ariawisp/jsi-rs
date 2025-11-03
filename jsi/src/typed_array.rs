use std::marker::PhantomData;

use crate::{
    array_buffer::JsiArrayBuffer, function::JsiFn, object::JsiObject, value::JsiValue, PropName,
    RuntimeHandle,
};

pub trait TypedArrayElement: Sized + Copy {
    const CONSTRUCTOR_NAME: &'static str;
}

impl TypedArrayElement for u8 {
    const CONSTRUCTOR_NAME: &'static str = "Uint8Array";
}

impl TypedArrayElement for i32 {
    const CONSTRUCTOR_NAME: &'static str = "Int32Array";
}

impl TypedArrayElement for f32 {
    const CONSTRUCTOR_NAME: &'static str = "Float32Array";
}

impl TypedArrayElement for f64 {
    const CONSTRUCTOR_NAME: &'static str = "Float64Array";
}

pub struct JsiTypedArray<'rt, T: TypedArrayElement> {
    pub(crate) object: JsiObject<'rt>,
    pub(crate) _phantom: PhantomData<T>,
}

impl<'rt, T: TypedArrayElement> JsiTypedArray<'rt, T> {
    pub fn new(length: usize, rt: &mut RuntimeHandle<'rt>) -> Self {
        let ctor_val = rt.global().get(PropName::new(T::CONSTRUCTOR_NAME, rt), rt);
        let ctor = ctor_val
            .try_into_js::<JsiFn>(rt)
            .expect("TypedArray constructor not found");

        let length_val = JsiValue::new_number(length as f64);
        let array_obj = ctor
            .call_as_constructor(std::iter::once(length_val), rt)
            .expect("TypedArray construction threw")
            .try_into_js::<JsiObject>(rt)
            .expect("TypedArray constructor did not return an object");

        Self {
            object: array_obj,
            _phantom: PhantomData,
        }
    }

    pub fn buffer(&self, rt: &mut RuntimeHandle<'rt>) -> JsiArrayBuffer<'rt> {
        self.object
            .get(PropName::new("buffer", rt), rt)
            .try_into_js(rt)
            .expect("TypedArray.buffer is not an ArrayBuffer")
    }

    pub fn as_slice(&self, rt: &mut RuntimeHandle<'rt>) -> &[T] {
        let buffer = self.buffer(rt);
        let data_ptr = buffer.data(rt).as_ptr() as *const T;
        let length = self
            .object
            .get(PropName::new("length", rt), rt)
            .0
            .get_number()
            .expect("TypedArray.length is not a number") as usize;

        unsafe { std::slice::from_raw_parts(data_ptr, length) }
    }

    pub fn as_mut_slice(&mut self, rt: &mut RuntimeHandle<'rt>) -> &mut [T] {
        let buffer = self.buffer(rt);
        let data_ptr = buffer.data(rt).as_mut_ptr() as *mut T;
        let length = self
            .object
            .get(PropName::new("length", rt), rt)
            .0
            .get_number()
            .expect("TypedArray.length is not a number") as usize;

        unsafe { std::slice::from_raw_parts_mut(data_ptr, length) }
    }
}

