# cadcad-rust-poc
Proof of Concept Rust Implementation of cadCAD

## 1. General Info

<h1>
  <img src="doc//cadCAD.rs_architecture.jpg" width="">  
</h1>

- All user config. (sim_config, init_state,  policies, state_update_fns) defined in Python and passed to Rust  
- Python policies/state update fns are called back from Rust (Rust state object also passed to Python because of this)  
- All library code `src/lib.rs` (run simulation loop, Rust-Python FFI etc..) is in Rust  


### What might be next?  

- [ ] [Speed Improv.] Currently, we call-back Python policies and state update functions from Rust with the help of Pyo3 library. For better performance, we might compile Python policies and state update functions (user config.) down to a low level shared library (e.g. to C using Cython) and call them in a more performant way. 

- [ ] [Speed Improv.] Currently, we use dictionaries (`PyDict`) from Pyo3 library as Hashmap containers (e.g. for State and Signals ). We might use faster Hashmaps (e.g. Fxhash or even Rust std::HashMap) for faster simulation runtimes.

- [ ] [All Types Support] Extend the State value type `enum Value` in `src/lib.rs` (see https://pyo3.rs/v0.15.1/conversions/tables.html#argument-types) to support more types between Rust-Python. Currently, only int32 and float64 types are supported.

- [ ] [Explore Other Solutions] Currently, we are using Pyo3 library and tools to achieve Rust-Python FFI which is the heart of the current solution, so we are potentially limited to Pyo3 capabilities/performance. We might research/experiment other options which might give us faster results.

- [x] [Speed Improv.] <Update: This task is done, improvement is huge, from 700ms to 270ms> Use single type `PyDict` for the type of State, remove `StatePy` and `StateRs` redundancy/conversion (the same already done for Signals and gave %33 speed improvement)

## 2. How to experiment

Install and Configure
```
Install Rust: https://www.rust-lang.org/tools/install
pip install virtualenv
git clone https://github.com/cadCAD-org/cadcad-rust-poc.git
cd cadcad-rust-poc
virtualenv --python python3.9 venv 
    // Windows: virtualenv env
source venv/bin/activate // or 'deactivate' when needed
    // Windows: env\Scripts\activate
pip install maturin
```

Build and Run
```
// From project root
cd <project_root>
maturin develop // debug build 
maturin develop --release // release build 
    // this cmd also creates `target/wheels/cadcad_rs-0.1.0-*-win_amd64.whl`
python3 config_prey_predator.py // run
```

Using cadcad_rs without virtual env. 
```
// This will install cadcad_rs in global Python scope
pip install target/wheels/cadcad_rs-0.1.0-*-win_amd64.whl
```

Shared lib. location:  
`<project_root>/venv/lib/python3.9/site-packages/cadcad_rs/cadcad_rs.cpython-39-darwin.so`


## 3. Performance Tests

### A. Perf. comparisons of different implementations with cadCAD.rs 

Comparing "the time to complete a simulation" with a sample user config. (reference Python impl. can be seen at the end of this section) used with different implementations:   

(The tests are done at commit https://github.com/cadCAD-org/cadcad-rust-poc/tree/4ced05351dd73078f3e785ce9d68466c3159c978)

| Implementation                   | Time to complete a simulation |
|----------------------------------|-------------------------------|
| 1. Everything in Rust <br/> &nbsp;&nbsp;&nbsp; (cadCAD.rs, this repo, used as an app.)               | ~84 ms                         |
| 2. Everything in Python <br /> &nbsp;&nbsp;&nbsp; (my very simple Python impl.)            | ~639 ms                        |
| 3. cadCAD.rs as library <br /> &nbsp;&nbsp;&nbsp; (this repo, unoptimized) | ~680 ms <- This Repo           |
| 4. Using cadCAD python package (v0.3.1)      | ~10.9 sec                        |

  
#### 1. Everything in Rust (cadCAD.rs, this repo, used as app.)
- All user config. (sim_config, init_state, policies, state_update_fns) and library code (run simulation loop etc.. ) are in Rust  
- How to experiment: 

```
cd cd perf_tests/pure_rust_impl
cargo r --release
```


#### 2. Everything in Python (my very simple Python impl.) 
- All user config. (sim_config, init_state,  policies, state_update_fns) and library code (run simulation loop etc.. ) are in Python  
- How to experiment: 

```
cd perf_tests/pure_python_impl
python main.py
```

#### 3. cadCAD.rs as library (this repo)    
- All user config. (sim_config, init_state,  policies, state_update_fns) defined in Python and passed to Rust  
- Python policies/state update fns are called back from Rust  
- All library code (run simulation loop etc.. ) is in Rust  
- How to experiment: See the related section for this repo above


#### 4. Using cadCAD python package   
- https://github.com/cadCAD-org/cadCAD
- How to experiment: 

```
cd cd perf_tests/using_cadCad_py_pkg
python main.py
```

#### The user config. used for performance tests: 

All implementations above used this user config:   
(Note: Pure Rust impl. uses Rust version of this config).

```py
import cadcad_rs, random

##
sim_config = {
    'T': 100_000,  # timesteps
    'N': 1,    # times the simulation will be run (Monte Carlo runs)
}

##
init_state = {
    'preys'    : 2000,
    'predators':  200.0, # This is float just to test software
}

## Params
MAX_PREYS = 3000

## Policies
def prey_change_normal_conditions(state, y):
    preys =  state['preys']    
    # Assuming: preys_change goes down with every iteration since
    # natural resources limits the number of preys to MAX_PREYS 
    preys_change = random.randint(0, MAX_PREYS-preys) if preys < MAX_PREYS else 0
    return ( "preys_change", preys_change )

def prey_pandemic(state, y):
    return ( "preys_change", random.randint(-800, -700) )

def predator_change_normal_conditions(state, y):
    return ( "predators_change", random.uniform(-10.0, 10.0) )

policies = [
    prey_change_normal_conditions,
    prey_pandemic, # enable to test addable signals
    predator_change_normal_conditions,
]

# SUFS/Mechanisms
def update_prey(state, signals):
    preys = state['preys'] + signals['preys_change']
    return ('preys', preys)

def update_predator(state, signals):
    predators = state['predators'] + signals['predators_change']
    return ('predators', predators)

state_update_fns = [
    update_prey,
    update_predator
]

result_data = cadcad_rs.run_simulation(
  "config from python",
  sim_config,
  init_state,
  policies,
  state_update_fns,
  print_trajectory
)
```   
Src: https://github.com/cadCAD-org/cadcad-rust-poc/blob/4ced05351dd73078f3e785ce9d68466c3159c978/config_prey_predator.py
    
Sample trajectory:		
```
State {'preys': 2000, 'predators': 200.0, 'run': 1, 'substep': 0, 'timestep': 0}
State {'preys': 2689, 'predators': 197.8061101157223, 'run': 1, 'substep': 1, 'timestep': 1}
State {'preys': 2905, 'predators': 202.0033859231905, 'run': 1, 'substep': 1, 'timestep': 2}
State {'preys': 2968, 'predators': 200.34499591706904, 'run': 1, 'substep': 1, 'timestep': 3}
State {'preys': 2978, 'predators': 198.70585863272157, 'run': 1, 'substep': 1, 'timestep': 4}
...
```

### B. Perf. compared - with and without pre-allocation:

**Summary:**    
Pre-allocated case is slightly faster in avarage  

Possible Next Actions: 
- Test with real life sized State object

State obj.:
```
INIT_STATE = [ 
    ("preys",     Foo { val: 2000, dummy_val: 0.1 } ),
    ("predators", Foo { val: 200 , dummy_val: 0.1 } )
]
```    

a) Final data and Trajectory vectors NOT pre-allocated:  

```
### Project: Example w/ struct value ...

---
 Starting simulation 0 ...
---
--- SIM_CONFIG: SimConfig { n_run: 3, timesteps: 500000 }
--- End of simulation 0
--- Elapsed time: 3.14s
--- Size of State obj.: 24
--- Size of traj. obj.: 12000024

---
 Starting simulation 1 ...
---
--- SIM_CONFIG: SimConfig { n_run: 3, timesteps: 500000 }
--- End of simulation 1
--- Elapsed time: 3.08s
--- Size of State obj.: 24
--- Size of traj. obj.: 12000024

---
 Starting simulation 2 ...
---
--- SIM_CONFIG: SimConfig { n_run: 3, timesteps: 500000 }
--- End of simulation 2
--- Elapsed time: 3.04s
--- Size of State obj.: 24
--- Size of traj. obj.: 12000024

----------------------END---------------------
```

b) Final data and Trajectory vectors pre-allocated:  

```
### Project: Example w/ struct value ...

---
 Starting simulation 0 ...
---
--- SIM_CONFIG: SimConfig { n_run: 3, timesteps: 500000 }
--- End of simulation 0
--- Elapsed time: 3.03s
--- Size of State obj.: 24
--- Size of traj. obj.: 12000024

---
 Starting simulation 1 ...
---
--- SIM_CONFIG: SimConfig { n_run: 3, timesteps: 500000 }
--- End of simulation 1
--- Elapsed time: 3.04s
--- Size of State obj.: 24
--- Size of traj. obj.: 12000024

---
 Starting simulation 2 ...
---
--- SIM_CONFIG: SimConfig { n_run: 3, timesteps: 500000 }
--- End of simulation 2
--- Elapsed time: 3.03s
--- Size of State obj.: 24
--- Size of traj. obj.: 12000024

----------------------END---------------------
```
### C. HashMap vs BTreeMap perf. test - with config_prey_predator_integer.rs:

**Summary:**   
For this example where we have small sized State object, using BTreeMap for State and Signal structs, we get the result with %38 less time compared to using HashMap.

Possible Next Actions: 
- Test with real life sized State object
- Try more efficient hash fns

```

//// State obj.
State::from([ ("preys", 2000), ("predators", 200), ] );

//// Tests

// Test 1 - Using "HashMap" for State and Signal structs

----------------------------------------------
### Project: Prey predators integer ...
---
 Starting simulation 0 ...
---
--- SIM_CONFIG: SimConfig { n_run: 2, timesteps: 1000000 }
--- End of simulation 0
--- Elapsed time: 8.07s
--- Size of State obj.: 48
--- Size of traj. obj.: 48000048
----------------------END---------------------

// Test 2 - Using "BTreeMap" for State and Signal structs

----------------------------------------------
### Project: Prey predators integer ...

---
 Starting simulation 0 ...
---
--- SIM_CONFIG: SimConfig { n_run: 2, timesteps: 1_000_000 }
--- End of simulation 0
--- Elapsed time: 5.15s
--- Size of State obj.: 24
--- Size of traj. obj.: 24_000_024
----------------------END---------------------
```
