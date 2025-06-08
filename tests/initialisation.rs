use std::path::PathBuf;

use img::Img;

fn initialisers() {
    let path = PathBuf::from("tests/assets/test.webp");

    let img: Img = path.try_into().unwrap();
}
