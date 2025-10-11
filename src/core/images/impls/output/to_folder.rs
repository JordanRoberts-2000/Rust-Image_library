use {
    crate::{Images, Result},
    fs_ext::dir,
    std::path::Path,
};

// impl Images {
//     pub fn to_folder(&self, path: impl AsRef<Path>) -> Result<()> {
//       let path = path.as_ref();
//       dir::ensure(path)?;

//       for image in &self.inner {
//         let dir = path.join(image.config.output_dir);
//         dir::ensure(path)?;
//         let path = dir.join(image.file_name());
//         image.save_as(path)
//       }
//     }
// }
