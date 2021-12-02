#![allow(non_snake_case)]
#![allow(dead_code)]

mod prey_predator_config;
use prey_predator_config::*;
use cadcad_rs::*;

fn main() {
    println!("\n################## cadCAD.rs ##################\n");

    run_simulation();

    println!("\n##################### END #####################\n\n\n");

    // println!("--- xx: {:?}", xx);
}

fn run_simulation() {

    // todo: create final_data - vec of traj.s

    for i in 0..SIM_CONFIG.n_run { // Simulation
        println!("--- \n Starting simulation {} ...", i);

        // 1. Display sim. config.
        println!("--- SIM_CONFIG: {:?}", SIM_CONFIG);

        // 2. Create trajectory
        let mut trajectory = vec![INIT_STATE.clone()];
        for i in 0..SIM_CONFIG.timesteps { // Experiment
            let current_state = &trajectory[i];
            let mut new_state = State::new();

            // a. Apply policies
            let mut signals = Signals::new();
            for policy in POLICIES {
                let signal = policy(current_state);
                signals.insert(signal.key, signal.value);
            }

            // b. Apply state update funcs
            for key_and_update_func in STATE_KEY_AND_UPDATE_FUNC_S {
                let update = (key_and_update_func.update_func)(current_state, &signals);
                new_state.insert(update.key, update.value);
            }

            trajectory.push(new_state);
        }

        // 3. Display result
        for (i, s) in trajectory.iter().enumerate() {
            println!("--- step {}: State {:?}", i, s);
        }
    }
}

