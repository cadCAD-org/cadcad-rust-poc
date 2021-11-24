#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::BTreeMap, usize};

fn main() {
    println!("\n##################### cadCAD.rs #####################\n");

    run_simulation();

    println!("\n##################### END #####################\n\n\n\n");
}

// -------------------- Config coming from client side e.g. python--------------- //

#[derive(Debug)]
struct SimConfig {
    n_run: usize,
    timesteps: usize
}

type State<'a> = BTreeMap<&'a str, i32>;
type UpdateFunc = fn(&State, &Signals) -> Update;
type PolicyFunc<'a> = fn(&'a State) -> Signals<'a>;
type Signals<'a> = BTreeMap<&'a str, i32>;

struct StateKeyAndUpdateFn {
    key: &'static str,
    update_func: UpdateFunc
}

#[derive(Debug)]
struct Update {
    key: &'static str,
    value: i32
}

// Policies
fn robot_arm_behavior<'a>(s: &State) -> Signals<'a> {
    let mut signals = Signals::new();
    let mut add_to_A = 0;
    if s["box_A"] > s["box_B"] {
        add_to_A = -1;
    }
    else if s["box_A"] < s["box_B"] {
        add_to_A = 1
    }
    signals.insert("add_to_A", add_to_A);
    signals.insert("add_to_B", -add_to_A);
    signals
}

// State update fns
fn update_box_A(s: &State, signals: &Signals) -> Update {
    let box_A = s["box_A"] + signals["add_to_A"];
    Update { key: "box_A", value: box_A}
}

fn update_box_B(s: &State, signals: &Signals) -> Update {
    let box_B = s["box_B"] + signals["add_to_B"];
    Update { key: "box_B", value: box_B}
}

// -------------------------- End of config -------------------------- //

fn run_simulation() {
    // -------------------- More client side config ---------------- //
    let sim_config = SimConfig { n_run: 1, timesteps: 10 };
    let init_state = State::from([ ("box_A", 10), ("box_B", 0) ]);
    let policyFns: Vec<PolicyFunc> = vec![
        robot_arm_behavior,
    ];    
    let state_key_and_update_func_s: Vec<StateKeyAndUpdateFn> = vec![
        StateKeyAndUpdateFn { key: "box_A", update_func: update_box_A },
        StateKeyAndUpdateFn { key: "box_B", update_func: update_box_B },
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
            let signals = robot_arm_behavior(current_state);
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

