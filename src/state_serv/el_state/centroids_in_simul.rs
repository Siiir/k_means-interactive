pub struct CentroidsStatesInSimul {
    step_to_els_groups: Vec<Vec<u8>>,
}
impl super::ElsStatesInSimul for CentroidsStatesInSimul {
    fn step_to_els_groups(&self) -> &Vec<Vec<u8>> {
        &self.step_to_els_groups
    }
    fn step_to_els_groups_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.step_to_els_groups
    }

    fn step<E, P>(&mut self, counterpart: &mut E, point_src: &P, simul_params: crate::SimulParams)
    where
        E: super::ElsStatesInSimul + 'static,
        P: crate::PointSrc,
    {
        // Useful bindings
        let points_states = {
            debug_assert_ne!(std::any::TypeId::of::<E>(), std::any::TypeId::of::<Self>());
            counterpart
        };
        let points_states_count = points_states.els_states_count();
        let centroid_states_count = self.els_states_count();

        // Current centroid stetes (groups) are defined by first looking at the points in curr_step.

        use std::cmp::Ordering as O;
        match centroid_states_count.cmp(&points_states_count) {
            O::Less => {
                debug_assert_eq!(1 + centroid_states_count, points_states_count);
                // Points are ahead. Nothing needs to be done.
            }
            O::Equal => {
                // Forcing correction step as it is needed.
                points_states.step(self, point_src, simul_params);
            }
            O::Greater => panic!("logical error: invalid argument passed to the function."),
        }
        let last_point_states = points_states
            .last_els_states()
            .expect("There should be some point states, due to forcing correction step.")
            .iter()
            .copied();

        todo!()
    }
}
