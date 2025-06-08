use crate::Img;

impl Img {
    pub fn blur(&mut self, blur: u8) -> &mut Self {
        self.img = self.img.blur(blur as f32);
        self
    }
}
