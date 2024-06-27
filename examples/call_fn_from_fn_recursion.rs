// FROM HERE
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

// Push and Pop
// https://doc.rust-lang.org/nomicon/vec/vec-push-pop.html

// use std::sync::{Mutex, OnceLock};
//use log::{debug, error, info, log_enabled, Level};
use log::{debug, error, info};
// use stdext::function_name;



fn main(){
    env_logger::init();
    
    error!("error");
    info!("info");
    debug!("debug");
    
    run();
}

fn run(){
    info!("start run");

    let max_recursion_steps = 1;
    let recursion = 0;

    let recursion = recursion_converse(max_recursion_steps,recursion);

    info!("recursion {}",recursion);

    info!("stop run");
}

fn recursion_converse(_max_recursion_steps:i32,mut recursion:i32) -> i32{

    recursion = recursion + 1;

    recursion 
}

// RUST_LOG=debug cargo run --example call_fn_from_fn_recursion