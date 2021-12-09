
// ------------------ Mimicking client side config e.g. Python--------------- //

use rand::Rng;
use cadcad_rs::*;
use std::ops::Add;
use std::ops::AddAssign;

// Simulation Config.
const SIM_CONFIG: SimConfig = SimConfig { n_run: 1, timesteps: 9 };

// Value Type
type ValueType = Value;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    F64(f64)
}

impl AddAssign for Value {
    fn add_assign(&mut self, other: Self) {
        match self {
            Self::I32(val) => {
                match other {
                    Self::I32(val_other) => { *self = Self::I32(*val + val_other) },
                    Self::F64(_) => panic!("-- Cannot add different enum types"),
                }
            },
            Self::F64(val) => {
                match other {
                    Self::I32(_) => panic!("-- Cannot add different enum types"),
                    Self::F64(val_other) => { *self = Self::F64(*val + val_other) }
                }                
            }
        };
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return match self {
            Self::I32(val) => {
                match other {
                    Self::I32(val_other) => Self::I32(val + val_other),
                    Self::F64(_) => panic!("-- Cannot add different enum types"),
                }
            },
            Self::F64(val) => {
                match other {
                    Self::I32(_) => panic!("-- Cannot add different enum types"),
                    Self::F64(val_other) => Self::F64(val + val_other),
                }
            }
        };
    }
}

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
    let preys_new = state["preys"] + signals["preys_change"];
    Update { key: "preys", value: preys_new }
}

fn update_predator(state: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let predators_new = state["predators"] + signals["predators_change"];
    Update { key: "predators", value: predators_new }
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
        init_state: &INIT_STATE,
        policies: POLICIES,
        state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
    };
}