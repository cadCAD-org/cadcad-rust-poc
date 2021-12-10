import cadcad_rs
print(cadcad_rs.sum_as_string(1, 2))
print(cadcad_rs.double(4))


##
sim_config = {
    'T': 5,  # timesteps
    'N': 1,   # times the simulation will be run (Monte Carlo runs)
}

init_state = {
    'preys': 2000,
    # 'box_B': 2,
}

cadcad_rs.run_simulation_rs(
  "config from python",
  sim_config,
  init_state
)