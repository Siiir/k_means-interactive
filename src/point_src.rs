use core::panic;
use std::rc::Rc;

pub use rand::IdentifiablePartRandPoints;
/// Error module
pub mod err;
pub mod rand;

type Point = [f32; 2];

/// Point source
///
/// # Constraints for all methods
/// * _Deterministic_ **when** called from the same object (in the same session).
pub trait PointSrc {
    /// Gets the requested point from the source returning their coordinates.
    fn get(&self, idx: usize) -> Result<Point, err::IdxOutOfBounds>;
    fn get_x(&self, idx: usize) -> Result<f32, err::IdxOutOfBounds> {
        self.get(idx).map(|coords| coords[0])
    }
    fn get_y(&self, idx: usize) -> Result<f32, err::IdxOutOfBounds> {
        self.get(idx).map(|coords| coords[1])
    }
    fn coord_x_getter_for_i32_idx(self: Rc<Self>) -> impl Fn(i32) -> f32 + 'static
    where
        Self: 'static,
    {
        coord_getter_for_i32_idx(self, 0)
    }
    fn coord_y_getter_for_i32_idx(self: Rc<Self>) -> impl Fn(i32) -> f32 + 'static
    where
        Self: 'static,
    {
        coord_getter_for_i32_idx(self, 1)
    }
}

/// # Causes panic
/// * Calling the provided getter on value that cannot be converted to `usize`.
fn coord_getter_for_i32_idx<S>(
    point_src: Rc<S>,
    coordinate_num: usize,
) -> impl Fn(i32) -> f32 + 'static
where
    S: ?Sized + PointSrc + 'static,
{
    use anyhow::Context;

    move |point_idx| {
        point_src
            .get(
                point_idx
                    .try_into()
                    .with_context(|| {
                        format!(
                "point indexer received invalid point index that cannot be converted to `usize`",
            )
                    })
                    .unwrap_or_else(|e| panic!("{e}")),
            )
            .unwrap_or_else(|e| panic!("{e}"))[coordinate_num]
    }
}
