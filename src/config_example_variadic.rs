
// ------------------ Mimicking client side config e.g. Python--------------- //

use rand::Rng;
use cadcad_rs::*;
use std::ops::Add;
use std::ops::AddAssign;

// Simulation Config.
const SIM_CONFIG: SimConfig = SimConfig { n_run: 1, timesteps: 9 };

// Value Type
type ValueType = Value;

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    F64(f64)
}

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        match self {
            Self::I32(val) => {
                match other {
                    Self::I32(val2) => { *self = Self::I32(*val + val2) },
                    Self::F64(_) => panic!("-- Mismatched underlying enum types"),
                }
            },
            Self::F64(val) => {
                match other {
                    Self::I32(_) => panic!("-- Mismatched underlying enum types"),
                    Self::F64(val2) => { *self = Self::F64(*val + val2) }
                }                
            }
        };
    }
}

// impl Add for Value {
//     type Output = Self;

//     fn add(self, other: Self) -> Self {
//         return match self {
//             Self::I32(val) => {
//                 match other {
//                     Self::I32(val2) => Self::I32(val + val2),
//                     Self::F64(_) => panic!("-- Mismatched underlying enum types"),
//                 }
//             },
//             Self::F64(val) => {
//                 match other {
//                     Self::I32(_) => panic!("-- Mismatched underlying enum types"),
//                     Self::F64(val2) => Self::F64(val + val2),
//                 }
//             }
//         };
//     }
// }

// Policies
fn prey_change_normal_conditions(_state: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let preys_change = random.gen_range(-100..100);
    Signal { key: "preys_change", value: Value::I32(preys_change) }
}

fn predator_change_normal_conditions(_state: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-10.0..10.0);
    Signal { key: "predators_change", value: Value::F64(predators_change) }
}

fn predator_pandemic(_state: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-1000.0..-50.0);
    Signal { key: "predators_change", value: Value::F64(predators_change) }
}

// State update fns
fn update_prey(state: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let mut preys = 42;
    if let Value::I32(val) = state["preys"] {
        preys = val;
    }
    let mut preys_change = 42;
    if let Value::I32(val) = signals["preys_change"] {
        preys_change = val;
    }
    let preys_new = preys + preys_change;
    Update { key: "preys", value: Value::I32(preys_new) }
}

fn update_predator(state: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let mut predators = 44.0;
    if let Value::F64(val) = state["predators"] {
        predators = val;
    }
    let mut predators_change = 44.0;
    if let Value::F64(val) = signals["predators_change"] {
        predators_change = val;
    }
    let predators_new = predators + predators_change;        
    Update { key: "predators", value: Value::F64(predators_new) }
}

// Init. State
lazy_static::lazy_static! {
    static ref INIT_STATE: State<'static, ValueType> = State::from(
        [
            ("preys",     Value::I32(2000)),
            ("predators", Value::F64(200.0)),
        ]
    );
}

// Mechanisms
const POLICIES: &'static [for<'r, 's> fn(&'r State<ValueType>) -> Signal<ValueType>] = &[
    prey_change_normal_conditions,
    predator_change_normal_conditions,
    // predator_pandemic
];

const STATE_KEYS_AND_UPDATE_FNS: &'static [StateKeyAndUpdateFn<ValueType>] = &[
    StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
    StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
];

lazy_static::lazy_static! {
    pub static ref CADCAD_CONFIG: cadCADConfig<'static, ValueType> = cadCADConfig {        
        name: "Prey predators integer",
        sim_config: SIM_CONFIG,
        init_state: (*INIT_STATE).clone(),
        policies: POLICIES,
        state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
    };
}