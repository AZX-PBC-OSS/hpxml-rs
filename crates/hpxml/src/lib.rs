#[cfg(not(any(feature = "v2", feature = "v3", feature = "v4", feature = "v5")))]
compile_error!(
    "At least one HPXML version feature must be enabled. \
     Example: `cargo add hpxml --features v4`"
);

pub use hpxml_core::*;
