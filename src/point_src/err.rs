use crate::MAX_POINT_COUNT;

#[derive(Debug, thiserror::Error)]
#[error("Index out of range. Maximum allowed index is {MAX_POINT_COUNT}.")]
pub struct IdxOutOfBounds;

pub fn validate_idx(idx: usize) -> Result<(), IdxOutOfBounds> {
    if idx <= crate::MAX_POINT_COUNT.get() {
        Ok(())
    } else {
        Err(IdxOutOfBounds)
    }
}
