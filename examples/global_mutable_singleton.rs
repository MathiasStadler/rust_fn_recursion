// FROM HERE
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton


use std::sync::{Mutex, OnceLock};

fn array() -> &'static Mutex<Vec<u8>> {
    static ARRAY: OnceLock<Mutex<Vec<u8>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

fn push_to_array() {
    array().lock().unwrap().push(1);
}

fn main(){

    run();
    run_one();
}

fn run() {
    push_to_array();
    push_to_array();
    push_to_array();
    push_to_array();

    println!("called {}", array().lock().unwrap().len());
}

fn run_one() {
    push_to_array();
    push_to_array();
    push_to_array();
    push_to_array();

    println!("called {}", array().lock().unwrap().len());
}