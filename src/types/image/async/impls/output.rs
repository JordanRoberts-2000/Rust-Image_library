use crate::Image;

// impl Image {
//   pub fn with_file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self {
//       self.config.file_name = file_name.as_ref().to_string();
//       self
//   }

//   pub fn prefix(&mut self, prefix: impl AsRef<str>) -> &mut Self {
//       self.config.prefix = Some(prefix.as_ref().to_string());
//       self
//   }

//   pub fn suffix(&mut self, suffix: impl AsRef<str>) -> &mut Self {
//       self.config.suffix = Some(suffix.as_ref().to_string());
//       self
//   }

//   pub fn file_name(&self) -> String {
//       format!(
//           "{}{}{}{}",
//           self.config.prefix.as_deref().unwrap_or(""),
//           self.config.file_name,
//           self.config.suffix.as_deref().unwrap_or(""),
//           self.format.extention()
//       )
//   }
// }

// pub fn extension(&self) -> &str {
//   self.format.extention()
// }

// pub fn file_stem(&self) -> String {
//   format!(
//       "{}{}{}",
//       self.config.prefix.as_deref().unwrap_or(""),
//       self.config.file_name,
//       self.config.suffix.as_deref().unwrap_or("")
//   )
// }

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
