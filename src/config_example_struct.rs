
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
}

impl AddAssign for Foo {
    fn add_assign(&mut self, other: Self) {
        *self = Self { val: self.val + other.val };
    }
}

// Policies
fn prey_change_normal_conditions(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let preys_change = random.gen_range(-100..100);
    Signal { key: "preys_change", value: Foo { val: preys_change } }
}

fn predator_change_normal_conditions(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut random = rand::thread_rng();
    let predators_change = random.gen_range(-10..10);
    Signal { key: "predators_change", value: Foo { val: predators_change } }
}

// State update fns
fn update_prey(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let preys_new = s["preys"].val + signals["preys_change"].val;
    Update { key: "preys", value: Foo { val: preys_new } }
}

fn update_predator(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
    let predators_new = s["predators"].val + signals["predators_change"].val;
    Update { key: "predators", value: Foo { val: predators_new } }
}

// Init. State
lazy_static::lazy_static! {
    static ref INIT_STATE: State<'static, ValueType> = State::from(
        [ 
            ("preys", Foo { val: 2000 } ),
            ("predators", Foo {val: 200 } )
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
        init_state: (*INIT_STATE).clone(),
        policies: POLICIES,
        state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
    };
}