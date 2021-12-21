import cadcad_rs, random

##
sim_config = {
    'T': 7,  # timesteps
    'N': 1,   # times the simulation will be run (Monte Carlo runs)
}

init_state = {
    'preys'    : 2000,
    'predators':  200.0,
}

# Policies
def prey_change_normal_conditions():
    return ( "preys_change", random.randint(-200, 200) )

def predator_change_normal_conditions():
    return ( "predators_change", random.uniform(-10.0, 10.0) )   

policies = [
    prey_change_normal_conditions, 
    predator_change_normal_conditions
]

print_trajectory = bool(0)

cadcad_rs.run_simulation_rs(
  "config from python",
  sim_config,
  init_state,
  policies,
  print_trajectory
)