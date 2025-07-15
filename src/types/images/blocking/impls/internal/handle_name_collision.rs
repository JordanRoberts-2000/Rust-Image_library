use std::collections::{HashMap, HashSet};

use crate::{blocking::Images, CollisionStrategy, ImageError, Result};

impl Images {
    pub(crate) fn handle_file_name_collisions(&mut self) -> Result<()> {
        match self.config.collision_strategy {
            CollisionStrategy::RemoveDuplicates => {
                let mut seen = HashSet::<String>::new();
                self.inner.retain(|image| seen.insert(image.file_name()));
                Ok(())
            }

            CollisionStrategy::Error => {
                let mut seen = HashSet::<String>::new();
                for image in &self.inner {
                    let filename = image.file_name();
                    if !seen.insert(filename.clone()) {
                        return Err(ImageError::FileNameCollision(filename));
                    }
                }
                Ok(())
            }

            CollisionStrategy::Dedupe => {
                let mut seen = HashMap::<String, usize>::new();
                for image in &mut self.inner {
                    let base = image.file_stem();
                    let count = seen.entry(base.clone()).or_insert(0);

                    if *count > 0 {
                        let name = format!("{}_{}", base, *count);
                        image.with_file_name(&name);
                    }
                    *count += 1;
                }
                Ok(())
            }
        }
    }
}
