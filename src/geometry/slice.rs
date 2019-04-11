use decorum::*;

use super::line::*;

#[derive(Debug, Clone)]
pub struct Slice {
    pub lines: Vec<Line>,
    pub height: R32,
}
impl Default for Slice {
    fn default() -> Slice {
        Slice {
            lines: Vec::new(),
            height: 0.0.into(),
        }
    }
}