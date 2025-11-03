use std::sync::Arc;

use crate::{sys, JsiObject, RuntimeHandle};

pub trait HybridObject: Send + Sync + 'static {
    fn hybrid_object_name(&self) -> &'static str;

    fn load_methods<'rt>(&self, obj: &mut JsiObject<'rt>, rt: &mut RuntimeHandle<'rt>);
}

pub trait HybridObjectExt: HybridObject + Sized {
    fn to_js_object<'rt>(self: Arc<Self>, rt: &'rt mut RuntimeHandle<'rt>) -> JsiObject<'rt>;
}

impl<T: HybridObject + 'static> HybridObjectExt for T {
    fn to_js_object<'rt>(self: Arc<Self>, rt: &'rt mut RuntimeHandle<'rt>) -> JsiObject<'rt> {
        let mut obj = JsiObject::new(rt);

        // Attach native state as Arc<Self> so methods can recover it from `this`.
        let boxed = sys::RustNativeState::from_arc(self.clone());
        let wrapper = sys::create_native_state_wrapper(boxed);
        sys::Object_setNativeState(obj.0.pin_mut(), rt.get_inner_mut(), wrapper);

        self.load_methods(&mut obj, rt);
        obj
    }
}
