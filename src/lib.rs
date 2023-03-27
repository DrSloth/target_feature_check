mod x86;

pub fn get_target_features() -> Vec<&'static str> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    get_x86_target_features()
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn get_x86_target_features() -> Vec<&'static str> {
    x86::get_target_features()
}

