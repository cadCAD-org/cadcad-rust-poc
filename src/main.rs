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

// Value Type
type ValueType = i32;
// type State_<'a> = State<'a, ValueType>;

// Policies
fn prey_policy(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut rng = rand::thread_rng();
    let preys_change = rng.gen_range(-100..100);
    Signal { key: "preys_change", value: preys_change }
}

fn predator_policy(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut rng = rand::thread_rng();
    let predators_change = rng.gen_range(-10..10);
    Signal { key: "predators_change", value: predators_change }
}

// State update fns
fn update_prey(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let preys_new = s["preys"] + signals["preys_change"];
    Update { key: "preys", value: preys_new}
}

fn update_predator(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let predators_new = s["predators"] + signals["predators_change"];
    Update { key: "predators", value: predators_new }
}

// -------------------------- End of config -------------------------- //

fn run_simulation() {
    // -------------------- More client side config ---------------- //
    let sim_config = SimConfig { n_run: 1, timesteps: 9 };
    let init_state = State::from([ ("preys", 2000), ("predators", 200) ]);
    let policies = [
        prey_policy, predator_policy
    ];
    let state_key_and_update_func_s: Vec<StateKeyAndUpdateFn<ValueType>> = vec![
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

type State<'a, T> = BTreeMap<&'a str, T>;
type UpdateFunc<T> = fn(&State<T>, &Signals<T>) -> Update<T>;
type PolicyFunc<'a, T> = fn(&State<T>) -> Signals<'a, T>;
type Signals<'a, T> = BTreeMap<&'a str, T>;

#[derive(Debug)]
struct SimConfig { 
    n_run: usize,
    timesteps: usize
}

struct StateKeyAndUpdateFn<T> {
    key: &'static str,
    update_func: UpdateFunc<T>
}

#[derive(Debug)]
struct Update<T> {
    key: &'static str,
    value: T
}

#[derive(Debug)]
struct Signal<T> {
    key: &'static str,
    value: T
}
