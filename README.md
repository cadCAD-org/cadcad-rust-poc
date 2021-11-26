# cadcad-rust-poc
Proof of Concept Rust Implementation of cadCAD


### How to experiment 
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
