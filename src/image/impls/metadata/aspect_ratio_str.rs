use crate::Image;

impl Image {
    pub fn aspect_ratio_str(&self) -> String {
        let gcd = Self::gcd(self.width, self.height);
        let x = self.width / gcd;
        let y = self.height / gcd;
        format!("{}:{}", x, y)
    }

    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
