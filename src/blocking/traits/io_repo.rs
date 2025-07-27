use {
    crate::Result,
    std::io::{Read, Write},
};

pub trait IoRepoOps {
    fn read_to_vec<R: Read>(&self, reader: R) -> Result<Vec<u8>>;
    // fn copy_to<R: Read, W: Write>(&self, reader: R, writer: W) -> Result<()>;
    // fn write_to_path<P: AsRef<Path>>(&self, path: P, data: &[u8]) -> Result<(), io::Error>;
}
