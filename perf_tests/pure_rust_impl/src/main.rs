#![allow(non_snake_case)]
#![allow(dead_code)]

use cadcad_rs::*;
use rand::Rng;

fn main() {
    println!("\n###################### cadCAD.rs ######################\n");

    let cadcad_config = create_config();
    run_simulation(&cadcad_config);
    
    println!("\n######################### END #########################\n\n\n");
}

// --------------------------- User config. code ------------------------- //

fn create_config() -> cadCADConfig<'static> {

    // Sim config.
    let sim_config = SimConfig { 
        n_run: 1,
        timesteps: 100_000
    };
    let print_trajectory = false;

    // Initial State
    let mut init_state = State::new();
    init_state.insert("preys".to_string(),     Value::I32(2000));
    init_state.insert("predators".to_string(), Value::F64(200.0));

    cadCADConfig {
        name: "Using pure Rust".to_string(),
        sim_config,
        init_state,
        policies: &[
            prey_change_normal_conditions,
            predator_change_normal_conditions
        ],
        state_key_and_update_fn_s: &[
            StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
            StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
        ],
        print_trajectory
    }
}

// Policies
fn prey_change_normal_conditions(_state: &State) -> Signal {
    let mut random = rand::thread_rng();
    let preys_change = random.gen_range(-100..100);
    Signal { key: "preys_change".to_string(), value: Value::I32(preys_change) }
}

fn predator_change_normal_conditions(_state: &State) -> Signal {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-10.0..10.0);
    Signal { key: "predators_change".to_string(), value: Value::F64(predators_change) }
}

// State update fns
fn update_prey(state: &State, signals: &Signals) -> Update {
    let preys_new = state["preys"] + signals["preys_change"];
    Update { key: "preys".to_string(), value: preys_new }
}

fn update_predator(state: &State, signals: &Signals) -> Update {
    let predators_new = state["predators"] + signals["predators_change"];
    Update { key: "predators".to_string(), value: predators_new }
}
