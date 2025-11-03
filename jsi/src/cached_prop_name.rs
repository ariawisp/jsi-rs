use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::{sys, PropName, RuntimeHandle};

thread_local! {
    static PROP_NAME_CACHE: RefCell<HashMap<&'static str, cxx::UniquePtr<sys::PropNameID>>> =
        RefCell::new(HashMap::new());
}

pub fn get_cached_prop_name<'rt>(name: &'static str, rt: &mut RuntimeHandle<'rt>) -> PropName<'rt> {
    PROP_NAME_CACHE.with(|tl| {
        let mut cache = tl.borrow_mut();
        let entry = cache
            .entry(name)
            .or_insert_with(|| sys::PropNameID_forUtf8(rt.get_inner_mut(), name));
        PropName(
            sys::PropNameID_copy(entry.as_ref().unwrap(), rt.get_inner_mut()),
            PhantomData,
        )
    })
}

pub fn clear_cache() {
    PROP_NAME_CACHE.with(|tl| tl.borrow_mut().clear());
}

#[macro_export]
macro_rules! prop {
    ($name:literal, $rt:expr) => {
        $crate::cached_prop_name::get_cached_prop_name($name, $rt)
    };
}
