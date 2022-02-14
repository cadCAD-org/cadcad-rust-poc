### This file is a dummy user config to use with performance tests
### Template used:
### https://github.com/cadCAD-org/demos/blob/master/tutorials/robots_and_marbles/videos/robot-marbles-part-3/config.py

# import libraries
from cadCAD.configuration import append_configs
from cadCAD.configuration.utils import config_sim
import random

sim_config = config_sim({
    'T': range(100_000), # number of discrete iterations in each experiment
    # 'T': range(10), # number of discrete iterations in each experiment
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

def prey_pandemic(_g, step, sL, state):
    return ( {"preys_change": random.randint(-800, -700)} )

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

# Build mechanism dictionary to "wire up the circuit"
mechanisms = [
    { 
        'policies': {
            'preys_change': prey_change_normal_conditions,
            'preys_change': prey_pandemic, # enable to test addable signals
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
    seeds={},
    raw_exogenous_states={},
    env_processes={},
    partial_state_update_blocks=mechanisms
)