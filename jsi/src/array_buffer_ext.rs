use std::marker::PhantomData;

use crate::{RuntimeHandle, array_buffer::JsiArrayBuffer, sys};

pub trait ArrayBufferExt {
    fn from_vec<'a>(vec: Vec<u8>, rt: &'a mut RuntimeHandle<'a>) -> JsiArrayBuffer<'a>;
}

impl ArrayBufferExt for JsiArrayBuffer<'_> {
    fn from_vec<'a>(mut vec: Vec<u8>, rt: &'a mut RuntimeHandle<'a>) -> JsiArrayBuffer<'a> {
        let len = vec.len();
        let capacity = vec.capacity();
        let ptr = vec.as_mut_ptr();

        // Leak the vector; memory will be reclaimed by the deleter
        std::mem::forget(vec);

        let deleter: sys::ExternalBufferDeleter = Box::new(move || unsafe {
            let _ = Vec::from_raw_parts(ptr, len, capacity);
        });
        let raw = Box::into_raw(Box::new(deleter)) as *mut sys::c_void;

        let buffer_ptr = unsafe {
            sys::Runtime_createArrayBufferFromExternal(
                rt.get_inner_mut(),
                ptr as *mut sys::c_void,
                len,
                raw,
            )
        };

        JsiArrayBuffer(buffer_ptr, PhantomData)
    }
}
