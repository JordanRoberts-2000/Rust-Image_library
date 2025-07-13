pub fn clamp_ratio(ratio: f32) -> f32 {
    if !(0.0..=1.0).contains(&ratio) {
        log::warn!("ratio: {} is outside [0.0,1.0], clamping to range", ratio);
        ratio.clamp(0.0, 1.0)
    } else {
        ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_ratio_within_bounds() {
        assert_eq!(clamp_ratio(0.0), 0.0);
        assert_eq!(clamp_ratio(0.5), 0.5);
        assert_eq!(clamp_ratio(1.0), 1.0);
    }

    #[test]
    fn test_clamp_ratio_below_zero() {
        let result = clamp_ratio(-0.25);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_clamp_ratio_above_one() {
        let result = clamp_ratio(1.25);
        assert_eq!(result, 1.0);
    }
}
