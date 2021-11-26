#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::BTreeMap, usize};
use rand::Rng;

fn main() {
    println!("\n################## cadCAD.rs ##################\n");

    run_simulation();

    println!("\n##################### END #####################\n\n\n");

    // println!("--- xx: {:?}", xx);
}

// -------------------- Config coming from client side e.g. python--------------- //

// Policies
fn prey_policy(_s: &State) -> Signal {
    let mut rng = rand::thread_rng();
    let preys_change = rng.gen_range(-100..100);
    Signal { key: "preys_change", value: preys_change }
}

fn predator_policy(_s: &State) -> Signal {
    let mut rng = rand::thread_rng();
    let predators_change = rng.gen_range(-10..10);
    Signal { key: "predators_change", value: predators_change }
}

// State update fns
fn update_prey(s: &State, signals: &Signals) -> Update {
    let preys = s["preys"] + signals["preys_change"];
    Update { key: "preys", value: preys}
}

fn update_predator(s: &State, signals: &Signals) -> Update {
    let predators = s["predators"] + signals["predators_change"];
    Update { key: "predators", value: predators }
}

// -------------------------- End of config -------------------------- //

fn run_simulation() {
    // -------------------- More client side config ---------------- //
    let sim_config = SimConfig { n_run: 1, timesteps: 9 };
    let init_state = State::from([ ("preys", 2000), ("predators", 200) ]);
    let policies = [
        prey_policy, predator_policy
    ];
    let state_key_and_update_func_s: Vec<StateKeyAndUpdateFn> = vec![
        StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
        StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
    ];
    // ------------------------------------------------------------ //

    for i in 0..sim_config.n_run { // Simulation
        println!("--- \n Starting simulation {} ...", i);

        // 1. Display sim config and initial state
        println!("--- sim_config: {:?}", sim_config);
        println!("--- init_state: {:?}", init_state);

        // 2. Create result
        let mut trajectory = vec![init_state.clone()];
        for i in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[i];
            let mut new_state = State::new();

            // a. Apply policies
            let mut signals = Signals::new();
            for policy in &policies {
                let signal = policy(current_state);
                signals.insert(signal.key, signal.value);
            }

            // b. Apply state update funcs
            for key_and_update_func in &state_key_and_update_func_s {
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

type State<'a> = BTreeMap<&'a str, i32>;
type UpdateFunc = fn(&State, &Signals) -> Update;
type PolicyFunc<'a> = fn(&State) -> Signals<'a>;
type Signals<'a> = BTreeMap<&'a str, i32>;

#[derive(Debug)]
struct SimConfig { 
    n_run: usize,
    timesteps: usize
}

struct StateKeyAndUpdateFn {
    key: &'static str,
    update_func: UpdateFunc
}

#[derive(Debug)]
struct Update {
    key: &'static str,
    value: i32
}

#[derive(Debug)]
struct Signal {
    key: &'static str,
    value: i32
}

