mod core;
mod impls {
    pub mod read_to_vec;
}

pub use core::IoRepo;
pub(super) use impls::read_to_vec::read_to_vec;
