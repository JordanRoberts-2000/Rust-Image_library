use std::path::PathBuf;

use img::Image;

fn initialisers() {
    let path = PathBuf::from("tests/assets/test.webp");

    let img: Image = path.try_into().unwrap();
}
