#![allow(non_snake_case)]
#![allow(dead_code)]

mod prey_predator_config;
use cadcad_rs::run_simulation;

fn main() {
    println!("\n################## cadCAD.rs ##################\n");

    run_simulation(&prey_predator_config::CADCAD_CONFIG);

    println!("\n##################### END #####################\n\n\n");
}