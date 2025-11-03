#pragma once

#include <jsi/jsi.h>
#include <rust/cxx.h>
#include <memory>

namespace jsi_rs::ffi {

struct RustNativeState;

class NativeStateWrapper final : public facebook::jsi::NativeState {
public:
  explicit NativeStateWrapper(rust::Box<RustNativeState> rust_state);
  ~NativeStateWrapper() override = default;

  // Returns a borrowed pointer to the inner Rust state.
  RustNativeState* get_rust_state() const;

private:
  mutable rust::Box<RustNativeState> rust_state_;
};

std::shared_ptr<facebook::jsi::NativeState> create_native_state_wrapper(
    rust::Box<RustNativeState> rust_state);

RustNativeState* extract_rust_native_state(
    const std::shared_ptr<facebook::jsi::NativeState>& state);

bool Object_hasNativeState(const facebook::jsi::Object& obj, facebook::jsi::Runtime& rt);

std::shared_ptr<facebook::jsi::NativeState> Object_getNativeState(
    const facebook::jsi::Object& obj,
    facebook::jsi::Runtime& rt);

void Object_setNativeState(
    facebook::jsi::Object& obj,
    facebook::jsi::Runtime& rt,
    std::shared_ptr<facebook::jsi::NativeState> state);

} // namespace jsi_rs::ffi

