use jsi::{prop, FromValue, IntoValue, JsiTypedArray};
use jsi_tests::harness::Harness;

#[test]
fn typed_array_write_read() {
    let mut h = Harness::new();
    h.run(|rt| {
        let mut ta = JsiTypedArray::<u8>::new(4, rt);
        {
            let s = ta.as_mut_slice(rt);
            s.copy_from_slice(&[9, 8, 7, 6]);
        }
        assert_eq!(ta.as_slice(rt), &[9, 8, 7, 6]);
    });
}

#[test]
fn prop_cache_reuse() {
    let mut h = Harness::new();
    h.run(|rt| {
        let mut o = rt.global();
        o.set(prop!("__testProp", rt), &123usize.into_value(rt), rt);
        let v = o.get(prop!("__testProp", rt), rt);
        let n = f64::from_value(&v, rt).unwrap();
        assert_eq!(n as usize, 123usize);
    });
}

// HybridObject is exercised via example crate; Unit test kept to non-hybrid APIs to avoid
// borrow invariance pitfalls in nested closures.
