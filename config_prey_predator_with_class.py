## Info:
## This file is a reference (and dummy) user config to use with performance tests

import cadcad_rs, random

##
sim_config = {
    # 'T': 100_000,  # timesteps
    'T': 10,  # timesteps
    'N': 1,   # times the simulation will be run (Monte Carlo runs)
}

##
# print_trajectory = bool(0)
print_trajectory = bool(1)

## 
class Preys:
	def __init__(self, population):
		self.population = population

	def __add__(self, rhs):
		return Preys(self.population + rhs.population)

	def __str__(self):
		return "Preys { %s }" % (self.population)

	def __repr__(self):
		return "Preys { %s }" % (self.population)

##
init_state = {
    'preys'    : Preys(2000),
    'predators':  200.0, # This is float just to test software
}

## Params
MAX_PREYS = 3000

## Policies
def prey_change_normal_conditions(state, y):
    preys =  state['preys']
    # Assuming: preys_change goes down with every iteration since
    # natural resources limits the number of preys to MAX_PREYS 
    preys_change = Preys(random.randint(0, MAX_PREYS-preys.population)) # if preys.population < MAX_PREYS else Preys(0)
    return ( "preys_change", preys_change )

def prey_pandemic(state, y):
    return ( "preys_change", Preys(random.randint(-200, -100)) )

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

cadcad_rs.run_simulation(
  "config from python",
  sim_config,
  init_state,
  policies,
  state_update_fns,
  print_trajectory
)