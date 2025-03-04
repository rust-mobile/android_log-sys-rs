use std::path::Path;

use bindgen::{
    EnumVariation, RustTarget, builder,
    callbacks::{EnumVariantCustomBehavior, ParseCallbacks},
};

fn main() {
    let ndk = std::env::var("ANDROID_NDK_ROOT").unwrap();
    let ndk = Path::new(&ndk);
    let log_header =
        ndk.join("toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/include/android/log.h");
    builder()
        .header(log_header.to_str().unwrap())
        .rust_target(RustTarget::stable(60, 0).map_err(|_| "Unexpected").unwrap())
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .parse_callbacks(Box::new(RenameCallback))
        .allowlist_item("(__)?android\\w+")
        .allowlist_item("log_id(_t)?")
        .generate()
        .unwrap()
        .write_to_file("src/bindings.rs")
        .unwrap()
}

#[derive(Debug)]
struct RenameCallback;
impl ParseCallbacks for RenameCallback {
    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        if original_variant_name == "LOG_ID_MIN" {
            Some(EnumVariantCustomBehavior::Hide)
        } else {
            None
        }
    }
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        if let Some(variant) = original_variant_name.strip_prefix("ANDROID_LOG_") {
            return Some(variant.to_owned());
        }
        if let Some(variant) = original_variant_name.strip_prefix("LOG_ID_") {
            return Some(variant.to_owned());
        }

        None
    }
}
