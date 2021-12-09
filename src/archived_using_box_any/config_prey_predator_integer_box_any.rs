
// ------------------ Mimicking client side config e.g. Python--------------- //

use rand::Rng;
use cadcad_rs::*;

// Simulation Config.
const SIM_CONFIG: SimConfig = SimConfig { n_run: 1, timesteps: 2 };

// Value Type
type ValueType = i32;

// Policies
fn prey_change_normal_conditions(_s: &State) -> Signal {
    let mut random = rand::thread_rng();
    let preys_change = random.gen_range(-100..100);
    Signal { key: "preys_change", value: preys_change }
}

fn predator_change_normal_conditions(_s: &State) -> Signal {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-10..10);
    Signal { key: "predators_change", value: predators_change }
}

fn predator_pandemic(_s: &State) -> Signal {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-1000..-50);
    Signal { key: "predators_change", value: predators_change }
}

// State update fns
fn update_prey(state: &State, signals: &Signals) -> Update {  
    let preys: &i32 = match state["preys"].downcast_ref::<i32>() {
        Some(val) => val,
        None => panic!("&a isn't a B!")
    };
    let preys_new = preys + signals["preys_change"];
    Update { key: "preys", value: Box::new(preys_new) }
}

fn update_predator(state: &State, signals: &Signals) -> Update {
    let predators: &i32 = match state["predators"].downcast_ref::<i32>() {
        Some(val) => val,
        None => panic!("&a isn't a B!")
    };    
    let predators_new = predators + signals["predators_change"];
    Update { key: "predators", value: Box::new(predators_new) }
}

// Init. State
// lazy_static::lazy_static! {
//     pub static ref INIT_STATE: State<'static> = State::from(
//         [
//             ("preys",     Box::new(2000)),
//             ("predators", Box::new(200)),
//         ]
//     );
// }

// use std::{collections::{BTreeMap}, usize};
// use std::any::Any;

// lazy_static::lazy_static! {
//     pub static ref INIT_STATE: State<'static> = {
//         let mut m = BTreeMap::new();
//         m.insert("preys", Box::<dyn Any + Send + Sync>::from(Box::new(2000)));
//         m
//     };
// }

// pub const INIT_STATE: &'static [i32] = &[1,2];

// Mechanisms
const POLICIES: &'static [for<'r, 's> fn(&'r State) -> Signal] = &[
    prey_change_normal_conditions,
    predator_change_normal_conditions,
    // predator_pandemic
];

const STATE_KEYS_AND_UPDATE_FNS: &'static [StateKeyAndUpdateFn] = &[
    StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
    StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
];

lazy_static::lazy_static! {
    pub static ref CADCAD_CONFIG: cadCADConfig<'static> = cadCADConfig {        
        name: "Prey predators integer",
        sim_config: SIM_CONFIG,
        // init_state: &INIT_STATE,
        policies: POLICIES,
        state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
    };
}