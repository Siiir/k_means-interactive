use std::ops::Deref;

use crate::SimulParams;

pub mod centroids_in_simul;
pub mod points_in_simul;

pub trait ElsStatesInSimul {
    // Required

    fn step_to_els_groups(&self) -> &Vec<Vec<u8>>;
    fn step_to_els_groups_mut(&mut self) -> &mut Vec<Vec<u8>>;
    fn step<E, P>(&mut self, counterpart: &mut E, point_src: &P, simul_params: SimulParams)
    where
        E: ElsStatesInSimul + 'static,
        P: crate::PointSrc;

    // Provided

    fn els_states_count(&self) -> usize {
        self.step_to_els_groups().len()
    }
    #[must_use]
    fn last_els_states(&self) -> Option<&[u8]> {
        self.step_to_els_groups().last().map(Vec::deref)
    }

    /// Returns the group.
    fn get<E, P>(
        &mut self,
        counterpart: &mut E,
        point_src: &P,
        simul_params: SimulParams,
        step: usize,
        point_idx: usize,
    ) -> u8
    where
        E: ElsStatesInSimul + 'static,
        P: crate::PointSrc,
    {
        self.ensure_idx_exists(counterpart, point_src, simul_params, step);
        self.step_to_els_groups_mut()[step][point_idx]
    }

    fn ensure_idx_exists<E, P>(
        &mut self,
        counterpart: &mut E,
        point_src: &P,
        simul_params: SimulParams,
        wanted_idx: usize,
    ) where
        E: ElsStatesInSimul + 'static,
        P: crate::PointSrc,
    {
        loop {
            if wanted_idx < self.step_to_els_groups_mut().len() {
                break;
            }
            self.step(counterpart, point_src, simul_params);
        }
    }
}
