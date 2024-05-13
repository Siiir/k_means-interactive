use std::cell::RefCell;

use rand::Rng;

use crate::PointSrc;

/// Will retrieve same points as were observed before or generate new observations.
#[derive(Default, Debug)]
pub struct IdentifiableRandPoints {
    observed: RefCell<Vec<[f32; 2]>>,
}

impl IdentifiableRandPoints {
    // CRUD-U: Extenders
    fn ensure_idx_exists(idx: usize, observed_mut: &mut std::cell::RefMut<'_, Vec<[f32; 2]>>) {
        let mut rng;
        loop {
            if idx < observed_mut.len() {
                break;
            }
            rng = rand::thread_rng();
            observed_mut.push(rng.gen());
        }
    }
}

// CRUD-U: Extending getters
impl PointSrc for IdentifiableRandPoints {
    /// Get the previous observation for provided `idx` or generates a new pseudo random one.
    ///
    /// # Returns
    /// * On the first call with `idx` = n: pseudo random point (x, y) where x, y in [0; 1).
    /// * On next calls with `idx` = n: same value as on the first call.
    fn get(self: &IdentifiableRandPoints, idx: usize) -> [f32; 2] {
        let mut observed_mut = self.observed.borrow_mut();

        IdentifiableRandPoints::ensure_idx_exists(idx, &mut observed_mut);
        observed_mut[idx]
    }
}
