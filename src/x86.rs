//! x86_64 and x86 feature detection

/// Add all detected features from the $feature list to the $store vector
macro_rules! add_detected_features {
    ($store:ident, $($feature:tt)*) => {
        $(
            if ::std::is_x86_feature_detected!($feature) {
                $store.push($feature);
            }
        )*
    }
}

/// Get the current detected x86 features.
pub fn get_target_features() -> Vec<&'static str> {
    // The features that are commented out are checkable but not recognized by rustc, maybe they
    // are just implied?
    let mut features = Vec::new();
    add_detected_features!(features,
        "aes"
        "pclmulqdq"
        "rdrand"
        "rdseed"
        // "tsc"
        // "mmx"
        "sse"
        "sse2"
        "sse3"
        "ssse3"
        "sse4.1"
        "sse4.2"
        "sse4a"
        "sha"
        "avx"
        "avx2"
        "avx512f"
        "avx512cd"
        "avx512er"
        "avx512pf"
        "avx512bw"
        "avx512dq"
        "avx512vl"
        "avx512ifma"
        "avx512vbmi"
        "avx512vpopcntdq"
        "avx512vbmi2"
        "gfni"
        "vaes"
        "vpclmulqdq"
        "avx512vnni"
        "avx512bitalg"
        "avx512bf16"
        "avx512vp2intersect"
        "f16c"
        "fma"
        "bmi1"
        "bmi2"
        // "abm"
        "lzcnt"
        "tbm"
        "popcnt"
        "fxsr"
        "xsave"
        "xsaveopt"
        "xsaves"
        "xsavec"
        "cmpxchg16b"
        "adx"
        "rtm"
    );
    features
}

