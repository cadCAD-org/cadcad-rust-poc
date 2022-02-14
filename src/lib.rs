#![allow(non_snake_case)]
#![allow(dead_code)]

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s
// Todo: Remove unnecessary prints after POC period

// Type Defs.
pub type State = PyDict;
pub type Trajectory<'a> = Vec<&'a State>;
pub type Signals = PyDict;
pub type UpdateFunc<'a> = &'a PyAny;
pub type PolicyFunc<'a> = &'a PyAny;

#[derive(Debug)]
pub struct SimConfig { 
    pub n_run: usize,
    pub timesteps: usize
}

// Create by state update fns
#[derive(Debug)]
pub struct Update<'a> {
    pub key: String,
    pub value: &'a PyAny
}

// Created by policies, used by state update fns
#[derive(Debug)]
pub struct Signal<'a> {
    pub key: String,
    pub value: &'a PyAny
}

#[allow(non_camel_case_types)]
pub struct cadCADConfig<'a> {
    pub name: String,
    pub sim_config: SimConfig,
    pub init_state: &'a State,
    pub policies: &'a PyList,
    pub state_update_functions: &'a PyList,
    pub print_trajectory: bool,
}

pub fn call_py_policy<'a>(
    policy: &'a PyAny, current_state: &State
) -> Signal<'a> {
    let pyPolicy = policy.downcast::<PyFunction>().unwrap();
    let result = pyPolicy.call1(
        (current_state, 0)
    ).unwrap().downcast::<PyTuple>().unwrap();
    let key = to_string(result.get_item(0).unwrap());
    let value = result.get_item(1).unwrap();  
    Signal { key, value }
}

pub fn call_py_state_update_fn<'a>(
    state_update_fn: &'a PyAny,
    current_state: &State,
    signals: &Signals
) -> Update<'a> {
    let pyfn = state_update_fn.downcast::<PyFunction>().unwrap();
    let result = pyfn.call1(
        (current_state, signals)
    ).unwrap().downcast::<PyTuple>().unwrap();
    let key = to_string(result.get_item(0).unwrap());
    let value = result.get_item(1).unwrap();
    Update { key, value }
}

// Pyo3 utility fns.
fn get_usize(dic: &PyDict, key: &str) -> usize {
    to_usize(dic.get_item(key).unwrap())
}

fn to_usize(any: &PyAny) -> usize {
    any.downcast::<PyInt>().unwrap().extract::<usize>().unwrap()
}

fn to_string(any: &PyAny) -> String {
    any.downcast::<PyString>().unwrap().extract::<String>().unwrap()
}

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

fn add_additional_init_state_keys(init_state: &State, i: usize) {
    let _todo = init_state.set_item("run", i+1);
    let _todo = init_state.set_item("substep", 0);
    let _todo = init_state.set_item("timestep", 0);
}

fn add_additional_new_state_keys(new_state: &State, i: usize, k: usize) {
    let _todo = new_state.set_item("run", i+1);
    let _todo = new_state.set_item("substep", 1);
    let _todo = new_state.set_item("timestep", k+1);
}

// Todo: Refactor this fn, remove unnecessary prints after POC period
fn run_simulation_impl(cadcad_config: &cadCADConfig) -> Vec<Vec<PyObject>> {
    let gil = Python::acquire_gil(); // Acquires the global interpreter lock, 
    let py = gil.python();           // allowing access to the Python interpreter.

    println!("\n----------------------------------------------");
    println!("\n### Project: {} ...", &cadcad_config.name);

    let module = PyModule::import(py, "operator").unwrap();
    let py_add = module.getattr("add").unwrap();

    // Final/result data set of simulation
    let mut result_data = Vec::<Vec<PyObject>>::new();
    let sim_config = &cadcad_config.sim_config;
    for i in 0..sim_config.n_run { // Simulation
        println!("\n--- \n Starting simulation {} ...", i);
        println!("---");
        // 1. Display sim. config.
        println!("--- SIM_CONFIG: {:?}", sim_config);

        let now = std::time::Instant::now(); // Perf. diag.
        // 2. Create trajectory
        let init_state = cadcad_config.init_state;
        let mut trajectory = vec![init_state];
        add_additional_init_state_keys(init_state, i);
        let mut trajectory_of_state_ptrs = vec![init_state.copy().unwrap().into()];

        for k in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[k];
            let new_state = State::new(py);

            // a. Apply policies
            let signals = Signals::new(py);
            for policy in cadcad_config.policies {
                let signal = call_py_policy(policy, current_state);
                // i. Add to the existing signal (to enable multiple Python
                //    policies for the same key writeable)
                if signals.contains(&signal.key).unwrap() {
                    let current_val = signals.get_item(&signal.key).unwrap();
                    let sum = py_add.call1( (current_val, signal.value) ).unwrap();
                    signals.set_item(&signal.key, sum)
                        .map_err(|err| println!("{:?}", err)).ok();
                }
                // ii. Insert a new signal
                else {
                    signals.set_item(&signal.key, signal.value)
                        .map_err(|err| println!("{:?}", err)).ok();
                }
            }

            // b. Apply state update fns
            for state_update_fn in cadcad_config.state_update_functions {
                let update = call_py_state_update_fn(
                    state_update_fn, current_state, &signals
                );
                new_state.set_item(update.key, update.value)
                    .map_err(|err| println!("{:?}", err)).ok();
            }

            add_additional_new_state_keys(new_state, i, k);
            trajectory.push(new_state);
            trajectory_of_state_ptrs.push(new_state.into());
        }

        // x. Perf. Diagnostics
        let elapsed = now.elapsed();
        println!("--- End of simulation {:?}", i);
        println!("--- Simulation time: {:.2?}", elapsed);

        // 3. Stats
        print_stats(&trajectory);

        // 4. Print trajectory
        if cadcad_config.print_trajectory { print_trajectory(&trajectory); }

        result_data.push(trajectory_of_state_ptrs);
    }
    println!("\n------------------ END of Simulation ---------------------\n");

    result_data
}

// ----------------------------------- pyo3 binding -------------------------------- //

use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
fn cadcad_rs(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m)]
    fn run_simulation(
        name: String,
        sim_config_py: &PyDict,
        init_state_py: &PyDict,
        policies_py: &PyList,
        state_update_fns_py: &PyList,
        print_trajectory: &PyBool
    ) -> PyResult<Vec::<Vec<PyObject>>> {
        let sim_config = SimConfig { 
            n_run: get_usize(sim_config_py, "N"),
            timesteps: get_usize(sim_config_py, "T")
        };
        let cadcad_config = cadCADConfig {
            name,
            sim_config,
            init_state: init_state_py,
            policies: policies_py,
            state_update_functions: state_update_fns_py,
            print_trajectory: print_trajectory.is_true(),
        };

        let result_data = run_simulation_impl(&cadcad_config);
        Ok(result_data)
    }

    Ok(())
}