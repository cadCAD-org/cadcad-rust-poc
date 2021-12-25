#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::{BTreeMap}, usize};
use std::ops::Add;
extern crate lazy_static;

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s

// State Value Type
type ValueType = Value;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    F64(f64),
}

impl Add for Value {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        // println!("--- other: {:?}", other);
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
type StringType = String; // todo: remove
// Todo: Consider HashMap later
pub type State = BTreeMap<StringType, ValueType>;
pub type StatePy<'a> = BTreeMap::<&'a str, PyObject>;
pub type Trajectory = Vec<State>;
pub type UpdateFunc = fn(&State, &Signals) -> Update;
// pub type PolicyFunc = fn(&State) -> Signal; // Rs
pub type PolicyFunc<'a> = &'a PyAny; // Py
pub type Signals = BTreeMap<StringType, ValueType>;

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
    pub key: StringType,
    pub value: ValueType
}

#[derive(Debug)]
pub struct Signal {
    pub key: StringType,
    pub value: ValueType
}

#[allow(non_camel_case_types)]
pub struct cadCADConfig<'a> {
    pub name: StringType,
    pub sim_config: SimConfig,
    pub init_state: State,
    // pub policies: &'a [PolicyFunc], // Rs
    pub policies: Vec<PolicyFunc<'a>>, // Py
    pub state_key_and_update_functions: &'a [StateKeyAndUpdateFn],
    pub print_trajectory: bool,
}

pub fn call_py_policy(pol: &PyAny, current_state_py: StatePy) -> Signal {
    let pyfn = pol.downcast::<PyFunction>().unwrap();
    let res = pyfn.call1((current_state_py, 0)).unwrap().downcast::<PyTuple>().unwrap();
    let key = to_string(res.get_item(0).unwrap());
    let val = res.get_item(1).unwrap();
    let val_type = val.get_type().to_string();
    let val = PY_TO_RUST.get(val_type.as_str())
        .expect("Unsupported python type")(val);    
    Signal { key, value: val }
}

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

type ToValueFn = for<'r> fn(&'r pyo3::PyAny) -> Value;
static PY_TO_RUST: phf::Map<&'static str, ToValueFn> = phf::phf_map! {
    "<class 'int'>"   => to_value_i32,
    "<class 'float'>" => to_value_f64,
};

fn print_trajectory(trajectory: &Trajectory) {
    println!("--- Trajectory:");
    for (i, state) in trajectory.iter().enumerate() {
        println!("---   step {}: State {:?}", i, state);
    }
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
        let mut trajectory = vec![cadcad_config.init_state.clone()];
        for i in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[i];
            let mut new_state = State::new();

            // a. Apply policies
            let mut signals = Signals::new();
            // for policy in cadcad_config.policies { // Rs
            //     let signal = policy(current_state); // Rs
            for policy in &cadcad_config.policies { // Py
                let mut current_state_py = StatePy::new();
                Python::with_gil(|py| {
                    for (key, val) in current_state {
                        match  val { 
                            Value::I32(i) => current_state_py.insert(key, i.to_object(py)), 
                            Value::F64(f) => current_state_py.insert(key, f.to_object(py)),
                        };
                    }
                });
                let signal = call_py_policy(policy, current_state_py); // Py end
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




// ----------------------------------- pyo3 ---------------------------------- //




use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
fn cadcad_rs(_py: Python, m: &PyModule) -> PyResult<()> {

    use rand::Rng;
    
    #[pyfn(m)]
    fn run_simulation_rs(
        name: String,
        sim_config_py: &PyDict,
        init_state_py: &PyDict,
        policies_py: &PyList,
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
        
        let mut policies: Vec<&PyAny> = Vec::new();
        for pol in policies_py {
            policies.push(pol);
        }

        let cadcad_config = cadCADConfig {
            name,
            sim_config,
            init_state,
            // policies: POLICIES, // Rs
            policies, // Py
            state_key_and_update_functions: STATE_KEYS_AND_UPDATE_FNS,
            print_trajectory: print_trajectory.is_true(),
        };

        run_simulation(&cadcad_config);

        Ok(42)
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

    // Mechanisms
    const POLICIES: &'static [for<'r, 's> fn(&'r State) -> Signal] = &[
        prey_change_normal_conditions,
        predator_change_normal_conditions,
    ];

    const STATE_KEYS_AND_UPDATE_FNS: &'static [StateKeyAndUpdateFn] = &[
        StateKeyAndUpdateFn { key: "preys", update_func: update_prey },
        StateKeyAndUpdateFn { key: "predators", update_func: update_predator },
    ];

    Ok(())
}