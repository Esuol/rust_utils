mod error;

use error::InitError;
pub use error::SoftBufferError;
use std::num::NonZeroU32;

/// A rectangular region of the buffer coordinate space.
#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// x coordinate of top left corner
    pub x: u32,
    /// y coordinate of top left corner
    pub y: u32,
    /// width
    pub width: NonZeroU32,
    /// height
    pub height: NonZeroU32,
}
