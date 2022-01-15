
// ------------------ Mimicking client side config e.g. Python--------------- //

use rand::Rng;
use cadcad_rs::*;
use std::ops::AddAssign;

// Simulation Config.
const SIM_CONFIG: SimConfig = SimConfig { n_run: 1, timesteps: 9 };

// Value Type
type ValueType = Foo;

#[derive(Clone, Debug)]
pub struct Foo {
    val: i32,
    dummy_val: f64,
}

impl AddAssign for Foo {
    fn add_assign(&mut self, other: Self) {
        *self = Self { 
            val: self.val + other.val,
            dummy_val: self.dummy_val + other.dummy_val,
        };
    }
}

// Policies
fn prey_change_normal_conditions(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let preys_change = random.gen_range(-100..100);
    Signal { 
        key: "preys_change", 
        value: Foo { val: preys_change, dummy_val: preys_change as f64 * 0.2 }
    }
}

fn predator_change_normal_conditions(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-10..10);
    Signal { 
        key: "predators_change", 
        value: Foo { val: predators_change, dummy_val: predators_change as f64 * 0.3 }
    }
}

// State update fns
fn update_prey(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let val_new = s["preys"].val + signals["preys_change"].val;
    let dummy_val_new = s["preys"].dummy_val + signals["preys_change"].dummy_val;
    Update { 
        key: "preys", 
        value: Foo { val: val_new, dummy_val: dummy_val_new }
    }
}

fn update_predator(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let val_new = s["predators"].val + signals["predators_change"].val;
    let dummy_val_new = s["predators"].dummy_val + signals["predators_change"].dummy_val;
    Update { 
        key: "predators", 
        value: Foo { val: val_new, dummy_val: dummy_val_new }
    }
}

// Init. State
lazy_static::lazy_static! {
    static ref INIT_STATE: State<'static, ValueType> = State::from(
        [ 
            ("preys",     Foo { val: 2000, dummy_val: 0.1 } ),
            ("predators", Foo { val: 200 , dummy_val: 0.1 } )
        ]
    );
}

// Mechanisms
const POLICIES: &'static [for<'r, 's> fn(&'r State<ValueType>) -> Signal<ValueType>] = &[
    prey_change_normal_conditions,
    predator_change_normal_conditions
];

const STATE_KEYS_AND_UPDATE_FNS: &'static [StateKeyAndUpdateFn<ValueType>] = &[
    StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
    StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
];

lazy_static::lazy_static! {
    pub static ref CADCAD_CONFIG: cadCADConfig<'static, ValueType> = cadCADConfig {        
        name: "Example w/ struct value",
        sim_config: SIM_CONFIG,
        init_state: &INIT_STATE,
        policies: POLICIES,
        state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
    };
}