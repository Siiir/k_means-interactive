pub struct PointsStatesInSimul {
    step_to_els_groups: Vec<Vec<u8>>,
}
impl super::ElsStatesInSimul for PointsStatesInSimul {
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
        let points_states_in_simul = {
            debug_assert!(std::any::TypeId::of::<E>() != std::any::TypeId::of::<Self>());
            counterpart
        };
        // prev_step == null
        if self.step_to_els_groups.is_empty() {
            push_0th_step_groups(&mut self.step_to_els_groups, simul_params);
        } else {
            // prev_step >= 0 ==> current_step > 0
            // this should be defined by first looking at the centroids in prev_step
            todo!()
        }
    }
}

fn push_0th_step_groups(step_to_els_groups: &mut Vec<Vec<u8>>, simul_params: crate::SimulParams) {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let curr_els_groups = std::iter::from_fn(|| Some(rng.gen_range(0..simul_params.group_count)))
        .take(simul_params.point_count)
        .collect();
    step_to_els_groups.push(curr_els_groups);
}
