use std::collections::HashMap;

pub mod simul_params;

pub use el_identity::ElIdentity;

use crate::SimulParams;
pub mod el_identity;

pub mod el_state;

pub struct StateServ {
    centroid_states: HashMap<u32, HashMap<u32, Vec<Vec<u32>>>>,
    point_states: HashMap<u32, HashMap<u32, Vec<Vec<u32>>>>,
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
            // self.centroid_states
            //     .entry(simul_params.group_count)
            //     .or_default()
            //     .entry(simul_params.point_count)
            //     .or_default();
            todo!()
        } else {
            todo!()
        }
    }
}
