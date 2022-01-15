## Info:
## This file consists of a dummy user config and
## a simple run simulation loop to be used for performance tests

import random, time

## ------- User config 
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
def prey_change_normal_conditions(state):
    preys =  state['preys']
    # Assuming: preys_change goes down with every iteration since
    # natural resources limits the number of preys to MAX_PREYS 
    preys_change = random.randint(0, MAX_PREYS-preys) if preys < MAX_PREYS else 0
    return ( "preys_change", preys_change )

def prey_migration(state):
    return ( "preys_change", random.randint(1000, 1010) )

def predator_change_normal_conditions(state):
    return ( "predators_change", random.uniform(-10.0, 10.0) )

policies = [
    prey_change_normal_conditions, 
    predator_change_normal_conditions,
    # prey_migration, # Enable to test aggregate-able policies feature
]

## SUFS/Mechanisms
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

## ----- Run simulation loop
print("\n### Sim. config:", sim_config)
trajectory = [init_state]
for i in range(sim_config['N']): # Simulation
    start = time.process_time()
    for k in range(sim_config['T']): # Experiment
        current_state = trajectory[k]

        signals = {}
        for pol in policies:
            signal = pol(current_state)
            key, val = signal
            if key in signals.keys():
                signals[key] = val + signals[key]
            else:
                signals[key] = val

        new_state = {}
        for state_update_fn in state_update_fns:
            update = state_update_fn(current_state, signals)
            new_state[update[0]] = update[1]
            
        trajectory.append(new_state)

    end = time.process_time()
    print("### Experiment took", end - start, "sec(s)\n")

##
if print_trajectory:
    for t in trajectory:
        print(t)

