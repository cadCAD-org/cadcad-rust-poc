use std::{collections::{BTreeMap}, usize};
use std::ops::Add;
extern crate lazy_static;

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s

// Todo: Consider HashMap later
pub type State<'a, T> = BTreeMap<&'a str, T>;
pub type UpdateFunc<T> = fn(&State<T>, &Signals<T>) -> Update<T>;
pub type PolicyFunc<T> = fn(&State<T>) -> Signal<T>;
pub type Signals<'a, T> = BTreeMap<&'a str, T>;

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
    pub key: &'static str,
    pub value: T
}

#[derive(Debug)]
pub struct Signal<T> {
    pub key: &'static str,
    pub value: T
}

#[allow(non_camel_case_types)]
pub struct cadCADConfig <'a, T: 'static> {
    pub name: &'static str,
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

extern crate libc;

use libc::size_t;
use std::slice;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    I32(i32),
    F64(f64)
}

// 1. Simple example
#[no_mangle]
pub extern "C" 
fn add(x: i32, y: i32) -> i32 {
    println!("--- xx: {:?}", Value::I32(x));

    x + y
}

// 2. Advanced example
#[no_mangle]
pub extern "C" 
fn sum_of_even(n: *const u32, len: size_t) -> u32 {
    let numbers = unsafe {
        assert!(!n.is_null());
        slice::from_raw_parts(n, len as usize)
    };

    numbers
        .iter()
        .filter(|&v| v % 2 == 0)
        .sum()
}

// x.