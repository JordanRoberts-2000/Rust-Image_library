#[derive(Debug, Clone, PartialEq, Copy, Eq)]
pub enum CropEdge {
    Left,
    Right,
    Top,
    Bottom,
    Horizontal,
    Vertical,
    All,
}
