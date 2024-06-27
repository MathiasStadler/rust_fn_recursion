// FROM HERE
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

// Push and Pop
// https://doc.rust-lang.org/nomicon/vec/vec-push-pop.html

// use std::sync::{Mutex, OnceLock};
//use log::{debug, error, info, log_enabled, Level};
use log::{debug, error, info};
// use stdext::function_name;

fn main() {
    // env_logger::init();
    init_logger();

    error!("error");
    info!("info");
    debug!("debug");

    run();
}

fn run() {
    info!("start run");

    let mut max_recursion_steps = 10;
    let recursion = 0;

    debug!("\t\t###");
    debug!("\t\t###");
    debug!("START CONVERSE RECURSION");
    let recursion = recursion_converse(max_recursion_steps, recursion);

    info!("recursion {}", recursion);

    max_recursion_steps = 10;

    debug!("\t\t###");
    debug!("\t\t###");
    debug!("START RECURSION ");
    let recursion = real_recursion(max_recursion_steps);

    info!("recursion {}", recursion);

    info!("stop run");
}

fn recursion_converse(max_recursion_steps: i32, mut recursion: i32) -> i32 {
    debug!("recursion Nr. => {}", recursion);
    debug!("max_recursion_steps => {}", max_recursion_steps);
    recursion = recursion + 1;

    if recursion < max_recursion_steps {
        info!("call next recursion");
        recursion_converse(max_recursion_steps, recursion);
    }

    recursion
}

fn real_recursion(start_recursion: i32) -> i32 {
    // init
    let mut recursion = start_recursion;
    debug!("recursion Nr. => {}", recursion);
    recursion = recursion - 1;

    if recursion > 0 {
        info!("call next recursion");
        real_recursion(recursion);
    }

    recursion
}

fn init_logger() {
    use std::io::Write;

    env_logger::builder()
        .format(|buf, record| {
            let warn_style = buf.default_level_style(log::Level::Info);
            let _timestamp = buf.timestamp();
            writeln!(
                buf,
                // FROM HERE
                // https://docs.rs/env_logger/latest/src/custom_format/custom_format.rs.html#35
                "{}:{}  {warn_style}{}{warn_style:#} - {}",
                // record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .init();
}

// RUST_LOG=debug cargo run --example call_fn_from_fn_recursion
