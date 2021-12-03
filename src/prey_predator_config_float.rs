
// ------------------ Mimicking client side config e.g. Python--------------- //

use rand::Rng;
use cadcad_rs::*;

// Simulation Config.
const SIM_CONFIG: SimConfig = SimConfig { n_run: 1, timesteps: 9 };

// Value Type
type ValueType = f64;

// Policies
fn prey_policy(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut rng = rand::thread_rng();
    let preys_change = rng.gen_range(-100.0..100.0);
    Signal { key: "preys_change", value: preys_change }
}

fn predator_policy(_s: &State<ValueType>) -> Signal<ValueType> {
    let mut rng = rand::thread_rng();
    let predators_change = rng.gen_range(-10.0..10.0);
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
lazy_static::lazy_static! {
    static ref INIT_STATE: State<'static, ValueType> = State::from(
        [ ("preys", 2000.0), ("predators", 200.0) ]
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

lazy_static::lazy_static! {
    pub static ref CADCAD_CONFIG: cadCADConfig<ValueType> = cadCADConfig {
        name: "Prey predators float",
        sim_config: SIM_CONFIG,
        init_state: (*INIT_STATE).clone(),
        policies: POLICIES,
        state_key_and_update_functions: STATE_KEY_AND_UPDATE_FUNC_S,
    };
}