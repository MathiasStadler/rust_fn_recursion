// FROM HERE
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

// Push and Pop
// https://doc.rust-lang.org/nomicon/vec/vec-push-pop.html

use std::sync::{Mutex, OnceLock};
use log::debug;
use stdext::function_name;

fn array() -> &'static Mutex<Vec<u8>> {
    static ARRAY: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

fn push_to_array(i:u8) {
    array().lock().unwrap().push(i);
}

fn pop_from_array() -> u8 {
    array().lock().unwrap().pop().expect("REASON")
}

fn main(){
    env_logger::init();
    run();
    run_one();
    // println!("HERE");
    debug!("debug");
    debug!("pop {}",pop_from_array());

    another_fn();
}

fn run() {
    push_to_array(1);
    push_to_array(2);
    push_to_array(3);
    push_to_array(4);

    println!("called {}", array().lock().unwrap().len());
}

fn run_one() {
    push_to_array(5);
    push_to_array(6);
    push_to_array(7);
    push_to_array(8);

    println!("called {}", array().lock().unwrap().len());
}

#[allow(dead_code)]
fn pop_vector() -> u8 {

    pop_from_array()
}

fn  another_fn(){

    debug!("another fn");
    
    debug!("pop {}",pop_from_array());
}