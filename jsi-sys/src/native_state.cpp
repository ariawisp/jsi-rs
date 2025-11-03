#include "native_state.h"

namespace jsi_rs::ffi {

NativeStateWrapper::NativeStateWrapper(rust::Box<RustNativeState> rust_state)
    : rust_state_(std::move(rust_state)) {}

RustNativeState* NativeStateWrapper::get_rust_state() const {
  // Borrow the inner Rust value without transferring ownership.
  return &*rust_state_;
}

std::shared_ptr<facebook::jsi::NativeState> create_native_state_wrapper(
    rust::Box<RustNativeState> rust_state) {
  return std::make_shared<NativeStateWrapper>(std::move(rust_state));
}

RustNativeState* extract_rust_native_state(
    const std::shared_ptr<facebook::jsi::NativeState>& state) {
  auto wrapper = std::dynamic_pointer_cast<NativeStateWrapper>(state);
  return wrapper ? wrapper->get_rust_state() : nullptr;
}

bool Object_hasNativeState(const facebook::jsi::Object& obj, facebook::jsi::Runtime& rt) {
  return obj.hasNativeState(rt);
}

std::shared_ptr<facebook::jsi::NativeState> Object_getNativeState(
    const facebook::jsi::Object& obj,
    facebook::jsi::Runtime& rt) {
  return obj.getNativeState(rt);
}

void Object_setNativeState(
    facebook::jsi::Object& obj,
    facebook::jsi::Runtime& rt,
    std::shared_ptr<facebook::jsi::NativeState> state) {
  obj.setNativeState(rt, state);
}

} // namespace jsi_rs::ffi

