use {
    crate::{
        blocking::{
            dependencies::ImageService,
            traits::{HttpClientOps, ImageServiceOps},
        },
        image::blocking::Image,
        ImageError, Result,
    },
    url::Url,
};

impl Image {
    pub fn from_url(url: impl AsRef<str>) -> Result<Self> {
        let url = Url::parse(url.as_ref())
            .map_err(|e| ImageError::UrlParse(e, url.as_ref().to_string()))?;

        Self::from_url_internal(url, &ImageService::default())
    }

    pub fn from_url_internal(url: Url, service: &impl ImageServiceOps) -> Result<Self> {
        let response = service.http().url(url)?;
        let (bytes, url) = service.http().parse_response(response)?;
        let metadata = service.metadata().from_bytes(&bytes)?;

        Ok(Self {
            src: ImageSrc::Url(url),
            data: Rc::new(RefCell::new(ImageData::EncodedBytes(bytes))),
            config: ImageConfig::default(),
            height: NonZeroU32::new(metadata.height).ok_or(ValidationError::InvalidHeight)?,
            width: NonZeroU32::new(metadata.width).ok_or(ValidationError::InvalidWidth)?,
            format: metadata.format,
        })
    }
}
