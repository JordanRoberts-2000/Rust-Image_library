mod core;
mod impls {
    pub mod atomic_write;
    pub mod check_exisitng_file;
    pub mod check_existing_dir;
    pub mod ensure_dir;
    pub mod get_file_size;
    pub mod trash_file;
}

pub use core::FsRepo;
pub(crate) use impls::{
    atomic_write::atomic_write, check_exisitng_file::check_existing_file,
    check_existing_dir::check_existing_dir, ensure_dir::ensure_dir, get_file_size::get_file_size,
    trash_file::trash_file,
};
