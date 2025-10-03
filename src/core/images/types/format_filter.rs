use {crate::ImageFormat, fs_ext::ExtensionFilter, std::collections::HashSet};

#[derive(Debug, Clone)]
pub enum FormatFilter {
    Allow(HashSet<ImageFormat>),
    Deny(HashSet<ImageFormat>),
}

impl FormatFilter {
    pub fn allow<I>(formats: I) -> Self
    where
        I: IntoIterator<Item = ImageFormat>,
    {
        Self::Allow(formats.into_iter().collect())
    }

    pub fn deny<I>(formats: I) -> Self
    where
        I: IntoIterator<Item = ImageFormat>,
    {
        Self::Deny(formats.into_iter().collect())
    }
}

impl From<FormatFilter> for ExtensionFilter {
    fn from(filter: FormatFilter) -> Self {
        match filter {
            FormatFilter::Allow(formats) => {
                let extensions = formats
                    .iter()
                    .flat_map(|fmt| fmt.extensions())
                    .map(|s| s.to_string())
                    .collect();
                ExtensionFilter::Allow(extensions)
            }
            FormatFilter::Deny(formats) => {
                let extensions = formats
                    .iter()
                    .flat_map(|fmt| fmt.extensions())
                    .map(|s| s.to_string())
                    .collect();
                ExtensionFilter::Deny(extensions)
            }
        }
    }
}
