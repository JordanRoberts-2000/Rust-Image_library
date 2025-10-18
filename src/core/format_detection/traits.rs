use {
    crate::{format_detection::Guesser, Result},
    std::{io::BufRead, path::Path},
    strum::IntoEnumIterator,
};

pub trait Guessable: Sized + IntoEnumIterator {
    fn guess(&self, bytes: &[u8]) -> bool;
    fn ext_guess(&self, ext: &str) -> bool;
    fn read_limit(&self) -> usize {
        512
    }

    fn guesser() -> Guesser<Self> {
        Guesser::<Self>::new()
    }

    fn guess_from_file(path: impl AsRef<Path>) -> Result<Option<Self>> {
        Guesser::<Self>::new().open(path)
    }

    fn guess_from_bytes(bytes: &[u8]) -> Option<Self> {
        Guesser::<Self>::new().from_bytes(bytes)
    }

    fn guess_from_reader<R: BufRead>(r: &mut R) -> Result<Option<Self>> {
        Guesser::<Self>::new().from_reader(r)
    }
}
