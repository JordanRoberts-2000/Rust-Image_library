mod clamp_ratio;
mod file_info;
mod gcd;
mod to_non_zero_u32;
mod encoding {
    pub mod encode_webp;
    pub mod resolve_avif_config;
    pub mod resolve_webp_config;
    pub mod to_rgba8;
}

pub use {
    clamp_ratio::clamp_ratio,
    encoding::{
        encode_webp::encode_webp_data, resolve_avif_config::resolve_avif_config,
        resolve_webp_config::resolve_webp_config, to_rgba8::to_rgba8_vec,
    },
    file_info::file_info,
    gcd::greatest_common_divisor,
    to_non_zero_u32::to_nonzero_u32_with_context,
};
