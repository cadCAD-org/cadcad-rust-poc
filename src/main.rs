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

fn update_box_A(s: &BTreeMap<&str, i32>) -> (&'static str, i32) {
    let mut add_to_A = 0;
    if s["box_A"] > s["box_B"] {
        add_to_A = -1;
    }
    else if s["box_A"] < s["box_B"] {
        add_to_A = 1
    }
    let box_A = s["box_A"] + add_to_A;

    ("box_A", box_A)
}

fn update_box_B(s: &BTreeMap<&str, i32>) -> (&'static str, i32) {
    let mut add_to_B = 0;
    if s["box_B"] > s["box_A"] {
        add_to_B = -1;
    }
    else if s["box_B"] < s["box_A"] {
        add_to_B = 1
    }
    let box_B = s["box_B"] + add_to_B;

    ("box_B", box_B)
}

// -------------------------- End of config -------------------------- //

fn next_state(changes: Vec<(&str, i32)>) -> BTreeMap<&str, i32> {
    let mut new_state = BTreeMap::new();
    for ch in &changes {
        new_state.insert(ch.0, ch.1);
    }
    new_state
}

fn run_simulation() {
    let sim_config = SimConfig { n_run: 2, timesteps: 10 };
    let init_state = BTreeMap::from([ ("box_A", 10), ("box_B", 0) ]);

    for i in 0..sim_config.n_run {
        println!("--- \n Starting simulation {} ...", i);

        // 1. Display sim config and initial state
        println!("--- sim_config: {:?}", sim_config);
        println!("--- init_state: {:?}", init_state);

        // 2. Create result
        let mut all_states = vec![init_state.clone()];
        for i in 0..sim_config.timesteps {
            let current_state = &all_states[i];
            let up_A = update_box_A(&current_state);
            let up_B = update_box_B(&current_state);
            let next_state = next_state(vec![up_A, up_B]);
            all_states.push(next_state);
        }

        // 3. Display result
        for (i, s) in all_states.iter().enumerate() {
        // for s in all_states {
            println!("--- step {}: {:?}", i, s);
        }
    }
}

