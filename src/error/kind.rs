use std::fmt;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Open,
    Read,
    ReadMetadata,
    Decode,
    ConformColor,
    Encode,
    Save,
    Serialize,
    Deserialize,
    Validate,
    FetchingUrl,
    Join,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        f.write_str(match self {
            Open => "opening",
            Read => "reading",
            ReadMetadata => "reading metadata",
            FetchingUrl => "fetching url",
            Decode => "decoding",
            ConformColor => "conforming color type",
            Encode => "encoding",
            Save => "saving",
            Serialize => "serializing",
            Deserialize => "deserializing",
            Validate => "validating",
            Join => "joining task",
        })
    }
}
