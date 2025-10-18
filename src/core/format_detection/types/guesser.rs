use {
    crate::{format_detection::Guessable, utils::normalise_ext, Result},
    std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
};

#[derive(Clone)]
pub struct Guesser<T: Guessable> {
    trust_extension: bool,
    extension_hint: Option<String>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Guessable> Default for Guesser<T> {
    fn default() -> Self {
        Self { trust_extension: true, _marker: std::marker::PhantomData, extension_hint: None }
    }
}

impl<T: Guessable> Guesser<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trust_extension(mut self, v: bool) -> Self {
        self.trust_extension = v;
        self
    }

    pub fn extension_hint(mut self, ext: Option<impl AsRef<str>>) -> Self {
        self.extension_hint = ext.map(|e| normalise_ext(e.as_ref()));
        self
    }

    pub fn from_bytes(&self, bytes: &[u8]) -> Option<T> {
        if let Some(ref ext) = self.extension_hint {
            if let Some(found) = T::iter().find(|v| v.ext_guess(ext)) {
                let limit = found.read_limit();
                let slice = &bytes[..bytes.len().min(limit)];
                if found.guess(slice) {
                    return Some(found);
                }
            }
        }

        T::iter().find(|v| {
            let limit = v.read_limit();
            let slice = &bytes[..bytes.len().min(limit)];
            v.guess(slice)
        })
    }

    pub fn from_reader<R: BufRead>(&self, r: &mut R) -> Result<Option<T>> {
        let buf = r.fill_buf()?;
        Ok(self.from_bytes(buf))
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<Option<T>> {
        let path = path.as_ref();

        if self.trust_extension {
            if let Some(os) = path.extension().and_then(|e| e.to_str()) {
                let ext = normalise_ext(os);
                if let Some(found) = T::iter().find(|v| v.ext_guess(&ext)) {
                    return Ok(Some(found));
                }
            }
        }

        let mut reader = BufReader::new(File::open(path)?);
        self.from_reader(&mut reader)
    }
}
