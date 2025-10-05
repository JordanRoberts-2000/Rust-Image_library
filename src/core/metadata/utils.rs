// greatest common divisor
pub fn gcd(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd_basic_cases() {
        assert_eq!(gcd(8, 12), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(10, 0), 10);
    }
}
