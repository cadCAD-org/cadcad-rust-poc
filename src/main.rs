#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::{collections::BTreeMap, usize};
use rand::Rng;

fn main() {
    println!("\n################## cadCAD.rs ##################\n");

    run_simulation();

    println!("\n##################### END #####################\n\n\n");

    // println!("--- xx: {:?}", xx);
}

// -------------------- Config coming from client side e.g. python--------------- //

// Simulation Config.
const SIM_CONFIG: SimConfig = SimConfig { n_run: 1, timesteps: 9 };

// Value Type
type ValueType = i32;

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

// Init. State
lazy_static! {
    static ref INIT_STATE: State<'static, ValueType> = State::from(
        [ ("preys", 2000), ("predators", 200) ]
    );
}

// Mechanisms
const POLICIES: &'static [for<'r, 's> fn(&'r State<ValueType>) -> Signal<ValueType>] = &[
    prey_policy, predator_policy
];

const STATE_KEY_AND_UPDATE_FUNC_S: &'static [StateKeyAndUpdateFn<ValueType>] = &[
    StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
    StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
];

// -------------------------- End of config -------------------------- //

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
