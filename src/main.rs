#![allow(non_snake_case)]
#![allow(dead_code)]

mod prey_predator_config_integer;
mod prey_predator_config_float;
use cadcad_rs::run_simulation;

fn main() {
    println!("\n################## cadCAD.rs ##################\n");

    run_simulation(&prey_predator_config_integer::CADCAD_CONFIG);
    run_simulation(&prey_predator_config_float::CADCAD_CONFIG);

    println!("\n##################### END #####################\n\n\n");
}