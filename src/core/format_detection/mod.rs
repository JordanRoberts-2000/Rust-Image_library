pub mod detect;
mod types {
    pub mod guesser;
    pub mod signature;
}
mod traits;

pub use {
    traits::Guessable,
    types::{guesser::Guesser, signature::Signature},
};
