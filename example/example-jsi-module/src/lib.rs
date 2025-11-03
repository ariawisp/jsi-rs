use jsi::{
    host_object, hybrid_method, hybrid_object,
    FromObject, FromValue, IntoValue, JsiFn, JsiObject, JsiString, JsiValue, PropName, RuntimeHandle,
    HybridObjectExt, AsValue,
};
use std::sync::{Arc, atomic::{AtomicI32, Ordering}};

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

pub fn init(rt: *mut jsi::sys::Runtime, call_invoker: cxx::SharedPtr<jsi::sys::CallInvoker>) {
    let (mut rt, _) = jsi::init(rt, call_invoker);

    let console = PropName::new("console", &mut rt);
    let console = rt.global().get(console, &mut rt);
    let console = JsiObject::from_value(&console, &mut rt).unwrap();

    let console_log = console.get(PropName::new("log", &mut rt), &mut rt);
    let console_log = JsiObject::from_value(&console_log, &mut rt).unwrap();
    let console_log = JsiFn::from_object(&console_log, &mut rt).unwrap();
    console_log
        .call(
            [JsiString::new("hello from Rust", &mut rt).into_value(&mut rt)],
            &mut rt,
        )
        .unwrap();

    // we called console.log("hello from Rust") using JSI! you should see the
    // log in your React Native bundler terminal

    // this is just an example, but from here, you could spawn threads or really
    // do whatever you want with the RuntimeHandle

    // make sure that any multithreaded operations use the CallInvoker if they
    // want to call back to JavaScript

    // now, for my next trick, I will add a host object to the global namespace
    let host_object = ExampleHostObject;
    let host_object = host_object.into_value(&mut rt);

    rt.global().set(PropName::new("ExampleGlobal", &mut rt), &host_object, &mut rt);

    let global_str = JsiString::new("hallo", &mut rt);
    let global_str = global_str.into_value(&mut rt);
    rt.global().set(PropName::new("ExampleGlobal2", &mut rt), &global_str, &mut rt);

    let global_num = JsiValue::new_number(3.200);
    rt.global().set(PropName::new("ExampleGlobal3", &mut rt), &global_num, &mut rt);

    // HybridObject example: Counter (construction example shown below)
    // let counter = Arc::new(Counter { value: AtomicI32::new(0) });
    // let counter_obj = counter.to_js_object(&mut rt);
    // rt.global().set(PropName::new("Counter", &mut rt), &counter_obj.into_value(&mut rt), &mut rt);
}

struct ExampleHostObject;

#[host_object]
impl ExampleHostObject {
    pub fn time(&self, _rt: &mut RuntimeHandle) -> anyhow::Result<i64> {
        Ok(3200)
    }
}

#[derive(Debug)]
struct Counter { value: AtomicI32 }

#[hybrid_object("Counter")]
impl Counter {
    #[hybrid_method]
    fn increment(&self, _rt: &mut RuntimeHandle) -> i32 {
        self.value.fetch_add(1, Ordering::SeqCst) + 1
    }

    #[hybrid_method]
    fn get_value(&self, _rt: &mut RuntimeHandle) -> i32 {
        self.value.load(Ordering::SeqCst)
    }
}
