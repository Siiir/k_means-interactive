use std::rc::Rc;

pub use rand::IdentifiableRandPoints;
pub mod rand;

/// Point source
///
/// # Constraints for all methods
/// * _Deterministic_ **when** called from the same object (in the same session).
pub trait PointSrc {
    /// Gets the requested point from the source returning their coordinates.
    fn get(&self, idx: usize) -> [f32; 2];
    fn get_x(&self, idx: usize) -> f32 {
        self.get(idx)[0]
    }
    fn get_y(&self, idx: usize) -> f32 {
        self.get(idx)[1]
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
/// * Calling the provided getter on negative `i32` value.
fn coord_getter_for_i32_idx<S>(
    point_src: Rc<S>,
    coordinate_num: usize,
) -> impl Fn(i32) -> f32 + 'static
where
    S: ?Sized + PointSrc + 'static,
{
    move |point_idx| {
        point_src.get(
            point_idx
                .try_into()
                .expect("point index should be non-negative"),
        )[coordinate_num]
    }
}
