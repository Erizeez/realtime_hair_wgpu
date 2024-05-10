mod methods;
mod utils;

use crate::physic_simulation::interfaces::SimulationTaskInterface;

pub fn do_der(task_interface: &mut SimulationTaskInterface) {
    let hairs = &mut task_interface.data.hairs;

    for strand in hairs.strands.iter_mut() {
        // Update reference frame
        for i in 0..strand.v_num {
            let e = strand.v_position[(i + 1) as usize] - strand.v_position[i as usize];
            let t = e.normalize();

            let reference_frame =
                utils::parallel_transport(strand.reference_frame[i as usize].t, t);
            strand.reference_frame[i as usize].b = reference_frame.0;
            strand.reference_frame[i as usize].n = reference_frame.1;
            strand.reference_frame[i as usize].t = t
        }

        // Calculate material frame

        // Apply stretch

        // Apply bend

        // Apply twist

        // Update strand states
    }
}
