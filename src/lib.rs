#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::{BTreeMap}, usize};
use std::ops::Add;
extern crate lazy_static;

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s
// Todo: Remove unnecessary prints after POC period

// State Value Type
#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    F64(f64),
    // ... this can be extended: https://pyo3.rs/v0.15.1/conversions/tables.html#argument-types
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

// Type Defs.
// Todo: Consider HashMap or other custom fast hashmap later
pub type State = BTreeMap<String, Value>;
pub type StatePy<'a> = BTreeMap::<&'a str, PyObject>;
pub type Trajectory = Vec<State>;
pub type UpdateFunc<'a> = &'a PyAny;
pub type PolicyFunc<'a> = &'a PyAny;
pub type Signals = BTreeMap<String, Value>;
pub type SignalsPy = BTreeMap::<String, PyObject>;

#[derive(Debug)]
pub struct SimConfig { 
    pub n_run: usize,
    pub timesteps: usize
}

// Create by state update fns
#[derive(Debug)]
pub struct Update {
    pub key: String,
    pub value: Value
}

// Created by policies, used by state update fns
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
    pub policies: Vec<PolicyFunc<'a>>,
    pub state_update_functions: Vec<UpdateFunc<'a>>,
    pub print_trajectory: bool,
}

pub fn call_py_policy(policy: &PyAny, current_state_py: StatePy) -> Signal {
    let pyPolicy = policy.downcast::<PyFunction>().unwrap();
    let result = pyPolicy.call1(
        (current_state_py, 0)
    ).unwrap().downcast::<PyTuple>().unwrap();
    let key = to_string(result.get_item(0).unwrap());
    let value = result.get_item(1).unwrap();
    let value_type = value.get_type().to_string();
    let value = PY_TO_RUST.get(value_type.as_str())
        .expect("Unsupported python type")(value);    
    Signal { key, value }
}

pub fn call_py_state_update_fn(
    state_update_fn: &PyAny,
    current_state_py: StatePy,
    signals: SignalsPy
) -> Update {
    let pyfn = state_update_fn.downcast::<PyFunction>().unwrap();
    let result = pyfn.call1(
        (current_state_py, signals)
    ).unwrap().downcast::<PyTuple>().unwrap();
    let key = to_string(result.get_item(0).unwrap());
    let value = result.get_item(1).unwrap();
    let value_type = value.get_type().to_string();
    let value = PY_TO_RUST.get(value_type.as_str())
        .expect("Unsupported python type")(value);    
    Update { key, value }
}

// Pyo3 utility fns.
fn get_i32(dic: &PyDict, key: &str) -> i32 {
    to_i32(dic.get_item(key).unwrap())
}

fn to_i32(any: &PyAny) -> i32 {
    any.downcast::<PyInt>().unwrap().extract::<i32>().unwrap()
}

fn to_f64(any: &PyAny) -> f64 {
    any.downcast::<PyFloat>().unwrap().extract::<f64>().unwrap()
}

fn to_string(any: &PyAny) -> String {
    any.downcast::<PyString>().unwrap().extract::<String>().unwrap()
}

fn to_value_i32(any: &PyAny) -> Value { Value::I32(to_i32(any)) }
fn to_value_f64(any: &PyAny) -> Value { Value::F64(to_f64(any)) }

// Python Rust type conversion map
type ToValueFn = for<'r> fn(&'r pyo3::PyAny) -> Value;
static PY_TO_RUST: phf::Map<&'static str, ToValueFn> = phf::phf_map! {
    "<class 'int'>"   => to_value_i32,
    "<class 'float'>" => to_value_f64,
    // ... this can be extended: https://pyo3.rs/v0.15.1/conversions/tables.html#argument-types
};

fn print_trajectory(trajectory: &Trajectory) {
    println!("--- Trajectory:");
    for (i, state) in trajectory.iter().enumerate() {
        println!("---   step {}: State {:?}", i, state);
    }
}

fn print_stats(trajectory: &Trajectory) {
    let size_of_state = std::mem::size_of::<State>();
    println!("--- Size of State obj.: {:?}", size_of_state);
    println!("--- Size of trajectory obj.: {}", std::mem::size_of_val(&*trajectory));
}

// Todo: Remove unnecessary prints after POC period
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
        let mut trajectory = vec![cadcad_config.init_state.clone()];
        for k in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[k];
            let mut new_state = State::new();

            // a. Apply policies
            let mut signals = Signals::new();
            let mut current_state_py = StatePy::new();
            for policy in &cadcad_config.policies {
                Python::with_gil(|py| {
                    for (key, val) in current_state {
                        match  val { 
                            Value::I32(i) => current_state_py.insert(key, i.to_object(py)), 
                            Value::F64(f) => current_state_py.insert(key, f.to_object(py)),
                        };
                    }
                });
                let signal = call_py_policy(policy, current_state_py.clone());
                // Insert new signal or update existing
                if let Some(mut_sig) = signals.get_mut(&signal.key) {
                    *mut_sig = *mut_sig + signal.value;
                }                
                else {
                    signals.insert(signal.key, signal.value);
                }
            }

            // b. Apply state update fns
            let mut signals_py = SignalsPy::new();
            Python::with_gil(|py| {
                for (key, val) in signals {
                    match  val { 
                        Value::I32(i) => signals_py.insert(key, i.to_object(py)), 
                        Value::F64(f) => signals_py.insert(key, f.to_object(py)),
                    };
                }
            });
            for state_update_fn in &cadcad_config.state_update_functions {
                let update = call_py_state_update_fn(
                    state_update_fn, current_state_py.clone(), signals_py.clone()
                );
                new_state.insert(update.key, update.value);
            }

            trajectory.push(new_state);
        }
        let elapsed = now.elapsed();
        println!("--- End of simulation {:?}", i);
        println!("--- Simulation time: {:.2?}", elapsed);

        // 3. Stats
        print_stats(&trajectory);

        // 4. Print trajectory
        if cadcad_config.print_trajectory { print_trajectory(&trajectory); }
    }
    println!("\n----------------------END---------------------\n");
}

// ----------------------------------- pyo3 binding -------------------------------- //

use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
fn cadcad_rs(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn run_simulation_rs(
        name: String,
        sim_config_py: &PyDict,
        init_state_py: &PyDict,
        policies_py: &PyList,
        state_update_fns_py: &PyList,
        print_trajectory: &PyBool
    ) -> PyResult<i32> {
        let sim_config = SimConfig { 
            n_run: get_i32(sim_config_py, "N") as usize,
            timesteps: get_i32(sim_config_py, "T") as usize
        };

        let mut init_state = State::new();
        for (key, val) in init_state_py.iter() {
            let key = to_string(key);
            let val_type = val.get_type().to_string();
            let val = PY_TO_RUST.get(val_type.as_str())
                .expect("Unsupported python type")(val);
            init_state.insert(key, val);
        }

        let policies : Vec<&PyAny> = policies_py.iter().collect();
        let state_update_fns : Vec<&PyAny> = state_update_fns_py.iter().collect();

        let cadcad_config = cadCADConfig {
            name,
            sim_config,
            init_state,
            policies,
            state_update_functions: state_update_fns,
            print_trajectory: print_trajectory.is_true(),
        };

        run_simulation(&cadcad_config);

        Ok(1)
    }

    Ok(())
}