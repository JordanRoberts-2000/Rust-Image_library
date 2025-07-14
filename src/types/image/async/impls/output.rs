use crate::Image;

impl Image {
    pub async fn file_name(&self, file_name: impl AsRef<str>) -> &Self {
        let mut state = self.state.write().await;
        state.config.file_name = file_name.as_ref().to_string();

        self
    }

    pub async fn prefix(&self, prefix: impl AsRef<str>) -> &Self {
        let mut state = self.state.write().await;
        state.config.prefix = Some(prefix.as_ref().to_string());

        self
    }

    pub async fn suffix(&self, suffix: impl AsRef<str>) -> &Self {
        let mut state = self.state.write().await;
        state.config.suffix = Some(suffix.as_ref().to_string());

        self
    }

    pub async fn build_file_name(&self) -> String {
        let state = self.state.read().await;
        let config = &state.config;
        format!(
            "{}{}{}",
            config.prefix.as_deref().unwrap_or(""),
            config.file_name,
            config.suffix.as_deref().unwrap_or("")
        )
    }
}
