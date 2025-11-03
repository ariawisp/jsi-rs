mod common;

use common::create_raw_runtime;
use jsi::{ArrayBufferExt, JsiArrayBuffer, RuntimeHandle};

#[test]
fn zero_copy_array_buffer_from_vec() {
    let mut raw = create_raw_runtime();
    let mut rt = unsafe { RuntimeHandle::from_raw_pin(raw.pin_mut()) };

    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let len = {
        let buf: JsiArrayBuffer = <JsiArrayBuffer as ArrayBufferExt>::from_vec(data, &mut rt);
        buf.data(&mut rt).len()
    };
    assert_eq!(len, 10);
}
