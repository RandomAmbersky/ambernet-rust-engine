mod point_2d;
mod size_2d;
mod array_2d;
mod delta_2d;

pub mod math;

pub use array_2d::Array2D as Array2D;
pub use point_2d::Point2dU32 as Point2D;
pub use size_2d::Size2dU32 as Size2D;

use delta_2d::Delta2dU32;
#[allow(dead_code)]
pub type Delta2D = Delta2dU32;
