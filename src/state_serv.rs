use std::collections::HashMap;

pub use simul_params::SimulParams;
pub mod simul_params;

pub use el_identity::ElIdentity;
pub mod el_identity;

pub struct StateServ {
    centroid_states: HashMap<u32, HashMap<u32, Vec<Vec<u32>>>>,
    point_states: HashMap<u32, HashMap<u32, Vec<Option<Vec<u32>>>>>,
    /// For these parameters, grouping must be kept in cache.
    current_params: SimulParams,
}

impl StateServ {
    /// This operation may take a while depending on how big the first 3 arguments are.
    pub fn group_num(
        &mut self,
        simul_params: SimulParams,
        step: u8, // `u8` restricts the `MAX` value.
        el_identity: ElIdentity,
    ) -> u32 {
        self.current_params = simul_params;
        let el_idx = el_identity.index();
        if el_identity.is_centroid() {
            todo!()
        } else {
            todo!()
        };
    }
}
