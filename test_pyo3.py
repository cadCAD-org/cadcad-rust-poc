import cadcad_rs
print(cadcad_rs.sum_as_string(1, 2))
print(cadcad_rs.double(4))

##
sim_config = {
    'T': 5,  # timesteps
    'N': 1,   # times the simulation will be run (Monte Carlo runs)
}

init_state = {
    'preys'    : 2000,
    'predators':  200.0,
}

cadcad_rs.run_simulation_rs(
  "config from python",
  sim_config,
  init_state
))

## Call this fn from Rust
def foo(x, y):
    print("--- foo() from python")
    return x+y
cadcad_rs.f1(foo)

