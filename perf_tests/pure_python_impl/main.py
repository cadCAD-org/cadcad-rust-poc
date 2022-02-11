## Info:
## This file consists of a dummy user config and
## a simple run simulation loop to be used for performance tests

import copy, random, time

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

def prey_pandemic(state):
    return ( "preys_change", random.randint(-800, -700) )

def predator_change_normal_conditions(state):
    return ( "predators_change", random.uniform(-10.0, 10.0) )

policies = [
    prey_change_normal_conditions,
    prey_pandemic, # enable to test addable signals
    predator_change_normal_conditions,    
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
def add_additional_init_state_keys(init_state, i):
    init_state["run"] = i+1;
    init_state["substep"] = 0;
    init_state["timestep"] = 0;    

def add_additional_new_state_keys(new_state, i, k):
    new_state["run"] = i+1;
    new_state["substep"] = 1;
    new_state["timestep"] = k+1;

trajectory = []
print("\n### Sim. config:", sim_config)
for i in range(sim_config['N']): # Simulation
    start = time.process_time()
    init_state_copy = copy.deepcopy(init_state)
    trajectory.append(init_state_copy)
    add_additional_init_state_keys(init_state_copy, i)

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
        add_additional_new_state_keys(new_state, i, k)
        trajectory.append(new_state)

    end = time.process_time()
    print("### One experiment took", end - start, "sec(s)\n")

##
if print_trajectory:
    for state in trajectory:
        print(state)

