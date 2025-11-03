#[cxx::bridge(namespace = "gpui::nitro::bridge")]
mod ffi {
    extern "Rust" {
        type RustButtonView;

        fn create_button_view(view_tag: i32) -> Box<RustButtonView>;
        fn destroy_button_view(view_tag: i32);

        fn button_get_disabled(self: &RustButtonView) -> bool;
        fn button_get_title(self: &RustButtonView) -> String;
        fn button_get_variant(self: &RustButtonView) -> String;
        fn button_get_size(self: &RustButtonView) -> String;
        fn button_get_loading(self: &RustButtonView) -> bool;

        fn button_set_disabled(self: &mut RustButtonView, disabled: bool);
        fn button_set_title(self: &mut RustButtonView, title: &str);
        fn button_set_variant(self: &mut RustButtonView, variant: &str);
        fn button_set_size(self: &mut RustButtonView, size: &str);
        fn button_set_loading(self: &mut RustButtonView, loading: bool);

        fn button_register_press_callback(self: &mut RustButtonView, callback_ptr: usize);
        fn button_get_state(self: &RustButtonView) -> RustButtonState;
        fn button_simulate_press(self: &RustButtonView);
    }

    #[derive(Debug)]
    struct RustButtonState {
        disabled: bool,
        loading: bool,
        pressed: bool,
        hovered: bool,
    }
}

use crate::button_view::{ButtonSize, ButtonVariant, ButtonView};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct RustButtonView {
    view: Arc<ButtonView>,
    #[allow(unused)]
    view_tag: i32,
    /// Optional native callback pointer (from C++). Currently unused; JS callbacks are preferred.
    native_cb: RwLock<Option<usize>>,
}

fn create_button_view(view_tag: i32) -> Box<RustButtonView> {
    let view = ButtonView::new();
    Box::new(RustButtonView { view, view_tag, native_cb: RwLock::new(None) })
}

fn destroy_button_view(_view_tag: i32) {
    // Nothing to do; RustButtonView will drop automatically when C++ releases the Box
}

fn button_get_disabled(this: &RustButtonView) -> bool {
    this.view.state.read().disabled
}

fn button_get_title(this: &RustButtonView) -> String {
    this.view.props.read().title.clone()
}

fn button_get_variant(this: &RustButtonView) -> String {
    this.view.props.read().variant.as_str().to_string()
}

fn button_get_size(this: &RustButtonView) -> String {
    this.view.props.read().size.as_str().to_string()
}

fn button_get_loading(this: &RustButtonView) -> bool {
    this.view.state.read().loading
}

fn button_set_disabled(this: &mut RustButtonView, disabled: bool) {
    this.view.props.write().disabled = disabled;
    this.view.state.write().disabled = disabled;
}

fn button_set_title(this: &mut RustButtonView, title: &str) {
    this.view.props.write().title = title.to_string();
}

fn button_set_variant(this: &mut RustButtonView, variant: &str) {
    this.view.props.write().variant = ButtonVariant::from(variant);
}

fn button_set_size(this: &mut RustButtonView, size: &str) {
    this.view.props.write().size = ButtonSize::from(size);
}

fn button_set_loading(this: &mut RustButtonView, loading: bool) {
    this.view.props.write().loading = loading;
    this.view.state.write().loading = loading;
}

fn button_register_press_callback(this: &mut RustButtonView, callback_ptr: usize) {
    *this.native_cb.write() = Some(callback_ptr);
}

fn button_get_state(this: &RustButtonView) -> ffi::RustButtonState {
    let st = this.view.state.read();
    ffi::RustButtonState { disabled: st.disabled, loading: st.loading, pressed: st.pressed, hovered: st.hovered }
}

fn button_simulate_press(this: &RustButtonView) {
    this.view.handle_press(false);
}

