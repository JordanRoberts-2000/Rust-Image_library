use crate::CollisionStrategy;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImagesConfig {
    pub flatten: bool,
    pub collision_strategy: CollisionStrategy,
}
