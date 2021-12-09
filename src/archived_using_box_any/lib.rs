use core::panic;
use std::any::Any;
use std::{collections::{BTreeMap}, usize};
extern crate lazy_static;

//// Improvements:
// Todo: Pre-allocate memory before everything (e.g. n_run * timesteps * sizeof State)
// Todo: Remove unnecessary "pub"s

// Todo: Consider HashMap later
// pub trait ValueTrait : Any + Clone + Sized {
//   fn method_foo(&self) -> Self where Self: Sized;
// }
pub trait Foo : Any + Send + Sync + std::fmt::Debug {}
pub type State<'a> = BTreeMap<&'a str, Box<dyn Any + Send + Sync>>;
pub type UpdateFunc = fn(&State, &Signals) -> Update;
pub type PolicyFunc = fn(&State) -> Signal;
pub type Signals<'a> = BTreeMap<&'a str, i32>;

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
    pub key: &'static str,
    pub value: Box<dyn Any + Send + Sync>
}

#[derive(Debug)]
pub struct Signal {
    pub key: &'static str,
    pub value: i32
}

#[allow(non_camel_case_types)]
pub struct cadCADConfig <'a> {
    pub name: &'static str,
    pub sim_config: SimConfig,
    // pub init_state: &'a State<'static>,
    pub policies: &'a [PolicyFunc],
    pub state_key_and_update_functions: &'a [StateKeyAndUpdateFn]
}

pub fn run_simulation(
    cadcad_config: &cadCADConfig
) {
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
        let mut init_state = State::new();
        init_state.insert("preys", Box::new(2000));
        init_state.insert("predators", Box::new(200));
        // let xx = cadcad_config.init_state;
        // for (k, v) in cadcad_config.init_state {
        //   println!("--- k: {}, v {:?}", k, v);
        // }
        let mut trajectory = vec![init_state];
        for i in 0..sim_config.timesteps { // Experiment
            let current_state = &trajectory[i];
            let mut new_state = State::new();

            // a. Apply policies
            let mut signals = Signals::new();
            for policy in cadcad_config.policies {
                let signal = policy(current_state);
                if let Some(mut_sig) = signals.get_mut(&signal.key) {
                    *mut_sig += signal.value;                    
                }                
                else {
                    signals.insert(signal.key, signal.value);
                }
            }

            // b. Apply state update funcs
            // let update = (cadcad_config.state_key_and_update_functions[0].update_func)(current_state, &signals);
            // new_state.insert(update.key, update.value);

            // let update = (cadcad_config.state_key_and_update_functions[1].update_func)(current_state, &signals);
            // new_state.insert(update.key, Box::new(update.value));

            for key_and_update_func in cadcad_config.state_key_and_update_functions {
                // println!("--- 11: {:?}", key_and_update_func);
                let update = (key_and_update_func.update_func)(current_state, &signals);
                // println!("--- 12: up: {:?}", &update);
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
        println!("--- Trajectory:");
        for (i, state) in trajectory.iter().enumerate() {
            print!("---   step {}: State: ", i);
            for (key, val) in state {        
                let any_inner = get_inner(val, &42);
                print!("( {}, {:?} ) ",  key, any_inner);
            }
            println!();
        }
    }
    println!("\n----------------------END---------------------\n");
}

fn get_inner<T: Any + Copy + std::fmt::Debug>(
    any: &Box<dyn Any + Send + Sync>, _type: &T
) -> T {
    match any.downcast_ref::<T>() {
        Some(val) => *val,
        None      => panic!("--- panic: todo"),
    }
}