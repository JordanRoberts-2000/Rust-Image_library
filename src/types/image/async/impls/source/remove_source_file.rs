use crate::Image;

impl Image {
    pub async fn remove_source_file(&self) -> &Self {
        let mut state = self.state.write().await;
        state.config.remove_source = true;
        self
    }
}
