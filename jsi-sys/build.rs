use std::{env, path::PathBuf};

fn main() {
    let base = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let pkg_base = PathBuf::from(base);

    let target_os = env::var_os("CARGO_CFG_TARGET_OS");
    let target_os = if let Some(s) = &target_os {
        s.to_str()
    } else {
        None
    };

    // Support external React Native via JSI_RS_RN_ROOT env var
    let rn_base = if let Some(rn_root) = env::var_os("JSI_RS_RN_ROOT") {
        PathBuf::from(rn_root)
    } else {
        pkg_base.join("vendor/react-native/packages/react-native")
    };

    let mut includes = vec![
        rn_base.join("ReactCommon/jsi"),
        rn_base.join("ReactCommon/callinvoker"),
        pkg_base.join("include"),
    ];

    // Desktop platforms (macOS/Linux/Windows) don't need React/Base
    if let Some("android") | Some("ios") = target_os {
        includes.push(rn_base.join("React"));
        includes.push(rn_base.join("React/Base"));
    }

    if let Some("android") = target_os {
        includes.push(
            rn_base.join("ReactAndroid/src/main/jni/react/turbomodule/"),
        );
        includes.push(pkg_base.join("vendor/fbjni/cxx"));
    }

    let includes: Vec<_> = IntoIterator::into_iter(includes)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing include path {:?}", p)))
        .collect();

    let mut compiles = vec![];

    // Only compile jsi.cpp if the feature is enabled (default: disabled for external RN usage)
    // Note: build scripts do not receive cfg(feature = ...) directly; Cargo exposes
    // enabled features as CARGO_FEATURE_<NAME> env vars.
    if std::env::var_os("CARGO_FEATURE_BUILTIN_JSI").is_some() {
        compiles.push(rn_base.join("ReactCommon/jsi/jsi/jsi.cpp"));
    }

    if let Some("android") = target_os {
        compiles.push(
            rn_base.join("ReactAndroid/src/main/jni/react/turbomodule/ReactCommon/CallInvokerHolder.cpp")
        );
    }

    let compiles: Vec<_> = IntoIterator::into_iter(compiles)
        .map(|p| dunce::canonicalize(&p).expect(&format!("missing compile file {:?}", p)))
        .collect();

    cxx_build::CFG
        .exported_header_dirs
        .extend(includes.iter().map(|e| e.as_path()));

    let mut bridges = vec!["src/ffi/base.rs", "src/ffi/host.rs"];

    if let Some("android") = target_os {
        bridges.push("src/ffi/android.rs");
    }

    for bridge in &bridges {
        println!("cargo:rerun-if-changed={}", bridge);
    }

    cxx_build::bridges(bridges)
        .flag_if_supported("-std=c++17")
        .files(compiles)
        .compile("jsi");

    println!("cargo:rerun-if-changed=include/wrapper.h");
    println!("cargo:rerun-if-changed=include/host.h");
    println!("cargo:rerun-if-env-changed=JSI_RS_RN_ROOT");
}
