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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_same_for_non_zero() {
        let v = to_nonzero_u32_with_context(42, "width");
        assert_eq!(v.get(), 42);
    }

    #[test]
    fn zero_becomes_one() {
        let v = to_nonzero_u32_with_context(0, "height");
        assert_eq!(v.get(), 1);
    }
}
