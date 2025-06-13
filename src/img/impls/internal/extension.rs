use crate::Img;

impl Img {
    pub fn update_extension(&mut self, new_ext: &str) {
        self.target_path = self.target_path.with_extension(new_ext);
    }
}
