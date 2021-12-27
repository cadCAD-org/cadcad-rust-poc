# import libraries
from cadCAD.configuration import append_configs
from cadCAD.configuration.utils import config_sim
import random

seeds = {
    # 'z': np.random.RandomState(1),
    # 'a': np.random.RandomState(2)
}

sim_config = config_sim({
    'T': range(100000), # number of discrete iterations in each experiement
    'N': 1, # number of times the simulation will be run (Monte Carlo runs)
})

## Params
MAX_PREYS = 3000

# Policies/Behaviors
def prey_change_normal_conditions(_g, step, sL, state):
    preys = state['preys']
    # Assuming: preys_change goes down with every iteration since
    # natural resources limits the number of preys to MAX_PREYS 
    preys_change = random.randint(0, MAX_PREYS-preys) if preys < MAX_PREYS else 0
    return ( {"preys_change": preys_change} )

def predator_change_normal_conditions(_g, step, sL, state):
    return ( {"predators_change": random.uniform(-10.0, 10.0)} )

# SUFS/Mechanisms
def update_prey(_g, step, sL, s, _input):
    preys = s['preys'] + _input['preys_change']
    return ('preys', preys)

def update_predator(_g, step, sL, s, _input):
    predators = s['predators'] + _input['predators_change']
    return ('predators', predators)    

# Initial States
genesis_states = {
    'preys': 2000,
    'predators': 200.0,
}

exogenous_states = {
    #'time': time_model
}

env_processes = {
}

#build mechanism dictionary to "wire up the circuit"
mechanisms = [
    { 
        'policies': {
            'preys_change': prey_change_normal_conditions,
            'predators_change': predator_change_normal_conditions
        },
        'variables': { # The following state variables will be updated simultaneously
            'preys': update_prey,
            'predators': update_predator
        }
    }
]

append_configs(
    sim_configs=sim_config,
    initial_state=genesis_states,
    seeds=seeds,
    raw_exogenous_states=exogenous_states,
    env_processes=env_processes,
    partial_state_update_blocks=mechanisms
)