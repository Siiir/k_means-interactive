/// Element identity
#[derive(Clone, Copy, Debug)]
pub struct ElIdentity {
    index: usize,
    is_centroid: bool,
}

impl ElIdentity {
    pub fn new(index: usize, is_centroid: bool) -> Self {
        Self { index, is_centroid }
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn is_centroid(&self) -> bool {
        self.is_centroid
    }
}
