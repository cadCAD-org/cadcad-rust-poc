#![allow(non_snake_case)]
#![allow(dead_code)]

mod config_prey_predator_integer;
mod config_prey_predator_float;
mod config_example_struct;
use cadcad_rs::run_simulation;

fn main() {
    println!("\n###################### cadCAD.rs ######################\n");

    run_simulation(&config_prey_predator_integer::CADCAD_CONFIG);
    run_simulation(&config_prey_predator_float::CADCAD_CONFIG);
    run_simulation(&config_example_struct::CADCAD_CONFIG);

    println!("\n######################### END #########################\n\n\n");
}