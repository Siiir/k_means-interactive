use derive_more::Constructor;

/// Simulation parameters
#[derive(Constructor, Clone, Copy, Debug)]
pub struct SimulParams {
    pub group_count: u32,
    pub point_count: u32,
}
