#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ArchiveFormat {
    TarGz,
    Tar,
    #[default]
    Zip,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CollisionStrategy {
    #[default]
    Dedupe,
    RemoveDuplicates,
    Error,
}
