#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;

/// Get the detected target features as a Vector
pub fn get_target_features() -> Vec<&'static str> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    get_x86_target_features()
}

/// Get the detected x86 target features as a Vector
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn get_x86_target_features() -> Vec<&'static str> {
    x86::get_target_features()
}

