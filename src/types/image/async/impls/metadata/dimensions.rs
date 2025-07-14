use crate::{image::utils::greatest_common_divisor, Image};

impl Image {
    pub async fn width(&self) -> u32 {
        self.state.read().await.width.get()
    }

    pub async fn height(&self) -> u32 {
        self.state.read().await.height.get()
    }

    pub async fn dimensions(&self) -> (u32, u32) {
        let state = self.state.read().await;
        (state.width.get(), state.height.get())
    }

    pub async fn aspect_ratio(&self) -> f32 {
        let state = self.state.read().await;
        state.width.get() as f32 / state.height.get() as f32
    }

    pub async fn aspect_ratio_str(&self) -> String {
        let state = self.state.read().await;
        let width = state.width.get();
        let height = state.height.get();
        let gcd = greatest_common_divisor(width, height);
        format!("{}:{}", width / gcd, height / gcd)
    }
}
