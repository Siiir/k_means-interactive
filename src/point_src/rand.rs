use std::{cell::RefCell, num::NonZeroUsize};

use rand::Rng;

use crate::PointSrc;

use super::err::IdxOutOfBounds;

/// Will retrieve same points as were observed before or generate new observations.
#[derive(Default, Debug)]
pub struct IdentifiablePartRandPoints {
    observed: RefCell<Vec<super::Point>>,
}

impl IdentifiablePartRandPoints {
    // Constants
    pub const BUF_GROWTH: NonZeroUsize = unsafe {
        // Safety:
        // The base literal is literally > 0 .
        NonZeroUsize::new_unchecked(1024)
    };

    // CRUD-C: Constructors
    pub fn with_base(base_collection: Vec<super::Point>) -> Self {
        Self {
            observed: RefCell::new(base_collection),
        }
    }

    // CRUD-R: Properties
    pub fn observed_count(&self) -> usize {
        self.observed.borrow().len()
    }

    // CRUD-U: Buffer extenders
    pub fn grow_buf(&self) {
        self.grow_buf_by(Self::BUF_GROWTH)
    }
    pub fn grow_buf_by(&self, growth: NonZeroUsize) {
        grow_buf_by(&mut self.observed.borrow_mut(), growth)
    }
}

// CRUD-U: Extending {getters & iterators}
impl PointSrc for IdentifiablePartRandPoints {
    /// # Causes panic
    /// If other `self`s method is invoked before returned iterator is consumed.
    fn iter<'s>(&'s self) -> impl Iterator<Item = super::Point> + 's {
        let mut observed = self.observed.borrow_mut();
        let mut next_buf_growth_on_idx = 0;
        let mut curr_idx = 0;
        std::iter::from_fn(move || -> Option<super::Point> {
            if curr_idx <= crate::MAX_POINT_COUNT.get() {
                if curr_idx == next_buf_growth_on_idx {
                    grow_buf(&mut observed);
                    next_buf_growth_on_idx =
                        next_buf_growth_on_idx.saturating_add(Self::BUF_GROWTH.get());
                }
                let curr_el = observed[curr_idx];
                curr_idx += 1;
                Some(curr_el)
            } else {
                None
            }
        })
    }
    /// Get the previous observation for provided `idx` or generates a new pseudo random one.
    ///
    /// # Returns
    /// * On the first call with `idx` = n: pseudo random point (x, y) where x, y in [0; 1).
    /// * On next calls with `idx` = n: same value as on the first call.
    fn get(&self, idx: usize) -> Result<super::Point, IdxOutOfBounds> {
        let mut observed_mut = self.observed.borrow_mut();

        ensure_idx_exists(&mut observed_mut, idx)?;
        Ok(observed_mut[idx])
    }
}

// CRUD-U: Buffer extenders
pub fn grow_buf(observed_mut: &mut std::cell::RefMut<'_, Vec<super::Point>>) {
    grow_buf_by(observed_mut, IdentifiablePartRandPoints::BUF_GROWTH)
}
pub fn grow_buf_by(
    observed_mut: &mut std::cell::RefMut<'_, Vec<super::Point>>,
    growth: NonZeroUsize,
) {
    let req_sure_idx = observed_mut.len() + growth.get() - 1;
    ensure_idx_exists(observed_mut, req_sure_idx).unwrap_or_default()
}
fn ensure_idx_exists(
    observed_mut: &mut std::cell::RefMut<'_, Vec<super::Point>>,
    idx: usize,
) -> Result<(), IdxOutOfBounds> {
    // Maybe is cached.
    if idx < observed_mut.len() {
        return Ok(());
    }
    // Should be valid if to be generated.
    super::err::validate_idx(idx)?;
    // Generate.
    let mut rng = rand::thread_rng();
    loop {
        observed_mut.push(rng.gen());
        if idx < observed_mut.len() {
            break Ok(());
        }
    }
}
