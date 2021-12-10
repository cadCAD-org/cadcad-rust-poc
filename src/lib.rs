#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::{BTreeMap}, usize};
use std::ops::Add;
extern crate lazy_static;

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s

// Todo: Consider HashMap later
type StringType = &'static str;
// type StringType = String;
pub type State<'a, T> = BTreeMap<StringType, T>;
pub type UpdateFunc<T> = fn(&State<T>, &Signals<T>) -> Update<T>;
pub type PolicyFunc<T> = fn(&State<T>) -> Signal<T>;
pub type Signals<'a, T> = BTreeMap<StringType, T>;

#[derive(Debug)]
pub struct SimConfig { 
    pub n_run: usize,
    pub timesteps: usize
}

pub struct StateKeyAndUpdateFn<T> {
    pub key: &'static str,
    pub update_func: UpdateFunc<T>
}

#[derive(Debug)]
pub struct Update<T> {
    pub key: StringType,
    pub value: T
}

#[derive(Debug)]
pub struct Signal<T> {
    pub key: StringType,
    pub value: T
}

#[allow(non_camel_case_types)]
pub struct cadCADConfig <'a, T: 'static> {
    pub name: StringType,
    pub sim_config: SimConfig,
    pub init_state: &'a State<'static, T>,
    pub policies: &'a [PolicyFunc<T>],
    pub state_key_and_update_functions: &'a [StateKeyAndUpdateFn<T>]
}

pub fn run_simulation<T>(
    cadcad_config: &cadCADConfig<T>
) where T: std::fmt::Debug + Clone + Copy + std::ops::Add + Add<Output = T> {
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
        let mut trajectory = vec![cadcad_config.init_state.clone()];
        for i in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[i];
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
            for key_and_update_func in cadcad_config.state_key_and_update_functions {
                let update = (key_and_update_func.update_func)(current_state, &signals);
                new_state.insert(update.key, update.value);
            }

            trajectory.push(new_state);
        }
        let elapsed = now.elapsed();
        println!("--- End of simulation {:?}", i);

        // x. Stats
        println!("--- Elapsed time: {:.2?}", elapsed);
        let size_of_state = std::mem::size_of::<State<T>>();
        println!("--- Size of State obj.: {:?}", size_of_state);
        println!("--- Size of traj. obj.: {}", std::mem::size_of_val(&*trajectory));

        // 3. Print trajectory
        println!("--- Trajectory:");
        for (i, s) in trajectory.iter().enumerate() {
            println!("---   step {}: State {:?}", i, s);
        }
    }
    println!("\n----------------------END---------------------\n");
}

// -------------------------------------------------------------------------- //

use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
fn cadcad_rs(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b + 10).to_string())
    }

    #[pyfn(m)]
    fn double(x: usize) -> usize {
        x * 2
    }

    // --------------------------

    // Value Type
    type ValueType = i32;
    // Policies
    use rand::Rng;
    pub fn prey_change_normal_conditions(_s: &State<ValueType>) -> Signal<ValueType> {
        let mut random = rand::thread_rng();
        let preys_change = random.gen_range(-100..100);
        Signal { key: "preys_change".to_string(), value: preys_change }
    }
    // State update fns
    fn update_prey(s: &State<ValueType>, signals: &Signals<ValueType>) -> Update<ValueType> {
        let preys_new = s["preys"] + signals["preys_change"];
        Update { key: "preys".to_string(), value: preys_new}
    }
    // Mechanisms
    const POLICIES: &'static [for<'r, 's> fn(&'r State<ValueType>) -> Signal<ValueType>] = &[
        prey_change_normal_conditions,
        // predator_change_normal_conditions,
        // predator_pandemic
    ];

    const STATE_KEYS_AND_UPDATE_FNS: &'static [StateKeyAndUpdateFn<ValueType>] = &[
        StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
        // StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
    ];    

    fn get_i32(dic: &PyDict, key: &str) -> i32 {
        dic.get_item(key).unwrap().downcast::<PyInt>().unwrap().extract::<i32>().unwrap()
    }
    
    #[pyfn(m)]
    fn run_simulation_rs(
        name: String,
        sim_config_py: &PyDict,
        init_state_py: &PyDict
    ) -> PyResult<i32> {
        let sim_config = SimConfig { 
            n_run: get_i32(sim_config_py, "N") as usize,
            timesteps: get_i32(sim_config_py, "T") as usize
        };

        let mut init_state = State::new();
        for e in init_state_py.iter() {
            let key = e.0.downcast::<PyString>().unwrap().extract::<String>().unwrap();
            let val = e.1.downcast::<PyInt>().unwrap().extract::<i32>().unwrap();
            init_state.insert(key, val);
        }

        let cadcad_config = cadCADConfig {
            name,
            sim_config,
            init_state: &init_state,
            policies: POLICIES,
            state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
        };

        run_simulation(&cadcad_config);

        Ok(42)
    }

    Ok(())
}