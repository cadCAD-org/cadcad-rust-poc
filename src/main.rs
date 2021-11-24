#![allow(non_snake_case)]

use std::{collections::BTreeMap, usize};

fn main() {
    println!("\n##################### cadCAD.rs #####################\n");

    run_simulation();
}

// -------------------- Config coming from client side e.g. python--------------- //

#[derive(Debug)]
struct SimConfig {
    n_run: usize,
    timesteps: usize
}

type State<'a> = BTreeMap<&'a str, i32>;
type UpdateFunc = fn(&State)->Update;

#[derive(Debug)]
struct Update {
    key: &'static str,
    value: i32
}

fn update_box_A(s: &State) -> Update {
    let mut add_to_A = 0;
    if s["box_A"] > s["box_B"] {
        add_to_A = -1;
    }
    else if s["box_A"] < s["box_B"] {
        add_to_A = 1
    }
    let box_A = s["box_A"] + add_to_A;

    Update { key: "box_A", value: box_A}
}

fn update_box_B(s: &State) -> Update {
    let mut add_to_B = 0;
    if s["box_B"] > s["box_A"] {
        add_to_B = -1;
    }
    else if s["box_B"] < s["box_A"] {
        add_to_B = 1
    }
    let box_B = s["box_B"] + add_to_B;

    Update { key: "box_B", value: box_B}
}

// -------------------------- End of config -------------------------- //

// fn next_state_0(updates: Vec<Update>) -> State<'static> {
//     let mut new_state = BTreeMap::new();
//     for up in &updates {
//         new_state.insert(up.key, up.value);
//     }
//     new_state
// }

// fn next_state<'a>(current_state: &'a State, mechanisms: &'a Vec<(&str, UpdateFunc)>) -> State<'a> {
//     let mut new_state = State::new();
//     for mech in mechanisms {
//         new_state.insert(mech.0, mech.1(current_state).value);
//     }
//     new_state
// }

fn run_simulation() {
    let sim_config = SimConfig { n_run: 2, timesteps: 10 };
    let init_state = State::from([ ("box_A", 10), ("box_B", 0) ]);
    let mechanisms: Vec<(&str, UpdateFunc)> = vec![
        ("box_A", update_box_A),
        ("box_B", update_box_B),
    ];
    
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
            for mech in &mechanisms {
                new_state.insert(mech.0, mech.1(current_state).value);
            }
            trajectory.push(new_state);
        }

        // 3. Display result
        for (i, s) in trajectory.iter().enumerate() {
            println!("--- step {}: {:?}", i, s);
        }
    }
}

