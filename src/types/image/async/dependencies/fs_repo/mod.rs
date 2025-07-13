mod core;
mod impls {
    pub mod check_exisitng_file;
    pub mod check_existing_dir;
    pub mod create_temp_file;
    pub mod ensure_dir;
    pub mod get_file_size;
    pub mod persist_temp_file;
    pub mod trash_file;
}

pub(super) use impls::{
    check_exisitng_file::check_existing_file, check_existing_dir::check_existing_dir,
    create_temp_file::create_temp_file, ensure_dir::ensure_dir, get_file_size::get_file_size,
    persist_temp_file::persist_temp_file, trash_file::trash_file,
};

pub use core::FsRepo;
