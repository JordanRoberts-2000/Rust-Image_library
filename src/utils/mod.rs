pub mod decode;
pub mod io;
mod http {
    pub mod r#async;
    pub mod blocking;
}

pub use http::blocking::BlockingHttpClient;
