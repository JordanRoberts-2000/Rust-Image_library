use {
    crate::{blocking::traits::IoRepoOps, Result},
    std::io::Read,
};

pub struct IoRepo;

impl IoRepoOps for IoRepo {
    fn read_to_vec<R: Read>(&self, reader: R) -> Result<Vec<u8>> {
        super::read_to_vec(reader)
    }
}
