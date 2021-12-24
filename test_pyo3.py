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
def prey_change_normal_conditions(state, y):
    max_preys = 3000
    preys =  state['preys']
    # Assuming: preys_change goes down with every iteration since 
    # natural resources are limits it to max_preys 
    preys_change = random.randint(0, max_preys-preys) if preys < max_preys else 0
    return ( "preys_change", preys_change )

def predator_change_normal_conditions(state, y):
    return ( "predators_change", random.uniform(-10.0, 10.0) )   

policies = [
    prey_change_normal_conditions, 
    predator_change_normal_conditions
]

print_trajectory = bool(1)

cadcad_rs.run_simulation_rs(
  "config from python",
  sim_config,
  init_state,
  policies,
  print_trajectory
)