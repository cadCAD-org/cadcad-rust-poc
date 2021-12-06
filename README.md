# cadcad-rust-poc
Proof of Concept Rust Implementation of cadCAD

## How to experiment 
git clone repo  
`cd cadcad-rust-poc`  
`cargo r`  

Example output:
```
##################### cadCAD.rs #####################

---
 Starting simulation 0 ...
--- sim_config: SimConfig { n_run: 1, timesteps: 10 }
--- init_state: {"predators": 100, "preys": 1000}
--- step 0: State {"predators": 100, "preys": 1000}
--- step 1: State {"predators": 101, "preys": 1075}
--- step 2: State {"predators": 97, "preys": 1123}
--- step 3: State {"predators": 90, "preys": 1086}
--- step 4: State {"predators": 99, "preys": 1083}
--- step 5: State {"predators": 105, "preys": 1156}
--- step 6: State {"predators": 99, "preys": 1221}
--- step 7: State {"predators": 105, "preys": 1173}
--- step 8: State {"predators": 103, "preys": 1217}
--- step 9: State {"predators": 111, "preys": 1244}
--- step 10: State {"predators": 113, "preys": 1166}

##################### END #####################
```

## Notes

### Performance



#### Y. Compared - with and without pre-allocation (6-Dec-21):

Summary:  
Pre-allocated case is slightly faster in avarage  

Possible Next Actions: 
- Test with real life sized State object

State obj.:
```
    static ref INIT_STATE: State<'static, ValueType> = State::from(
        [ 
            ("preys",     Foo { val: 2000, dummy_val: 0.1 } ),
            ("predators", Foo { val: 200 , dummy_val: 0.1 } )
        ]
    );
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
#### Z. HashMap vs BTreeMap test - with config_prey_predator_integer.rs (5-Dec-21):

Summary:  
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
