## Info:
## This file is a reference (and dummy) user config to use with performance tests

import cadcad_rs, random

##
sim_config = {
    'T': 100_000,  # timesteps
    'N': 1,   # times the simulation will be run (Monte Carlo runs)
}

##
print_trajectory = bool(0)

##
init_state = {
    'preys'    : 2000,
    'predators':  200.0,
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

def predator_change_normal_conditions(state, y):
    return ( "predators_change", random.uniform(-10.0, 10.0) )   

policies = [
    prey_change_normal_conditions, 
    predator_change_normal_conditions
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

cadcad_rs.run_simulation_rs(
  "config from python",
  sim_config,
  init_state,
  policies,
  state_update_fns,
  print_trajectory
)