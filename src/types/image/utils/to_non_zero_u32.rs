use std::num::NonZeroU32;

pub fn to_nonzero_u32_with_context(value: u32, context: &str) -> NonZeroU32 {
    match NonZeroU32::new(value) {
        Some(v) => v,
        None => {
            log::warn!("{} cannot be 0, using 1 instead", context);
            // Safety: 1 is guaranteed to be non-zero
            unsafe { NonZeroU32::new_unchecked(1) }
        }
    }
}
