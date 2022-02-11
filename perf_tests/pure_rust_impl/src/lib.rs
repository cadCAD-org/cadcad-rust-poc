use std::{collections::{BTreeMap}, usize};
use std::ops::Add;

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s

// State Value Type
#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    F64(f64),
    USIZE(usize),
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        return match self {
            Self::I32(val) => {
                match other {
                    Self::I32(val_other) => Self::I32(val + val_other),
                    Self::F64(_) => panic!("-- Cannot add different enum types"),
                    Self::USIZE(_) => panic!("-- Cannot add different enum types"),
                }
            },
            Self::F64(val) => {
                match other {
                    Self::I32(_) => panic!("-- Cannot add different enum types"),
                    Self::F64(val_other) => Self::F64(val + val_other),
                    Self::USIZE(_) => panic!("-- Cannot add different enum types"),
                }
            }
            Self::USIZE(val) => {
                match other {
                    Self::I32(_) => panic!("-- Cannot add different enum types"),
                    Self::F64(_) => panic!("-- Cannot add different enum types"),
                    Self::USIZE(val_other) => Self::USIZE(val + val_other),
                }
            }            
        };
    }
}

// Type Defs.
// Todo: Consider HashMap later
pub type State = BTreeMap<String, Value>;
pub type Trajectory = Vec<State>;
pub type UpdateFunc = fn(&State, &Signals) -> Update;
pub type PolicyFunc = fn(&State) -> Signal;
pub type Signals = BTreeMap<String, Value>;

#[derive(Debug)]
pub struct SimConfig { 
    pub n_run: usize,
    pub timesteps: usize
}

pub struct StateKeyAndUpdateFn {
    pub key: &'static str,
    pub update_func: UpdateFunc
}

#[derive(Debug)]
pub struct Update {
    pub key: String,
    pub value: Value
}

#[derive(Debug)]
pub struct Signal {
    pub key: String,
    pub value: Value
}

#[allow(non_camel_case_types)]
pub struct cadCADConfig<'a> {
    pub name: String,
    pub sim_config: SimConfig,
    pub init_state: State,
    pub policies: &'a [PolicyFunc],
    pub state_key_and_update_fn_s: &'a [StateKeyAndUpdateFn],
    pub print_trajectory: bool,
}

fn print_trajectory(trajectory: &Trajectory) {
    println!("--- Trajectory:");
    for (i, state) in trajectory.iter().enumerate() {
        println!("---   step {}: State {:?}", i, state);
    }
}

fn add_additional_init_state_keys(init_state: &mut State, i: usize) {
    let _todo = init_state.insert("run".to_string(), Value::USIZE(i+1));
    let _todo = init_state.insert("substep".to_string(), Value::USIZE(0));
    let _todo = init_state.insert("timestep".to_string(), Value::USIZE(0));
}

fn add_additional_new_state_keys(new_state: &mut State, i: usize, k: usize) {
    let _todo = new_state.insert("run".to_string(), Value::USIZE(i+1));
    let _todo = new_state.insert("substep".to_string(), Value::USIZE(1));
    let _todo = new_state.insert("timestep".to_string(), Value::USIZE(k+1));
}

pub fn run_simulation(cadcad_config: &cadCADConfig) {
    // todo: create final_data - vec of traj.s
    let sim_config = &cadcad_config.sim_config;
    println!("----------------------------------------------");
    println!("\n### Project: {} ...", &cadcad_config.name);
    for i in 0..sim_config.n_run { // Simulation
        println!("\n--- \n Starting simulation {} ...", i);
        println!("---");
        // 1. Display sim. config.
        println!("--- SIM_CONFIG: {:?}", sim_config);

        let now = std::time::Instant::now();
        // 2. Create trajectory
        let mut init_state = cadcad_config.init_state.clone();
        add_additional_init_state_keys(&mut init_state, i);
        let mut trajectory = vec![init_state];
        for k in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[k];
            let mut new_state = State::new();

            // a. Apply policies
            let mut signals = Signals::new();
            for policy in cadcad_config.policies {
                let signal = policy(current_state);
                if let Some(mut_sig) = signals.get_mut(&signal.key) {
                    *mut_sig = *mut_sig + signal.value;
                }                
                else {
                    signals.insert(signal.key, signal.value);
                }
            }

            // b. Apply state update funcs
            for key_and_update_fn in cadcad_config.state_key_and_update_fn_s {
                let update = (key_and_update_fn.update_func)(current_state, &signals);
                new_state.insert(update.key, update.value);
            }
            add_additional_new_state_keys(&mut new_state, i, k);

            trajectory.push(new_state);
        }
        let elapsed = now.elapsed();
        println!("--- End of simulation {:?}", i);

        // x. Stats
        println!("--- Elapsed time: {:.2?}", elapsed);
        let size_of_state = std::mem::size_of::<State>();
        println!("--- Size of State obj.: {:?}", size_of_state);
        println!("--- Size of traj. obj.: {}", std::mem::size_of_val(&*trajectory));

        // 3. Print trajectory
        if cadcad_config.print_trajectory {
            print_trajectory(&trajectory);
        }

    }
    println!("\n----------------------END---------------------\n");
}