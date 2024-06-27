// FROM HERE
// https://users.rust-lang.org/t/async-fn-recursion-rust-1-77/108779

#[allow(unused_imports)]
use log::{debug, error, info, log_enabled, Level};
use std::io::Write;

use stdext::function_name;




fn main() {


    env_logger::builder()
        .format(|buf, record| {
            let warn_style = buf.default_level_style(log::Level::Warn);
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


    //println!("Hello, second recursion!");

    info!("Hello, second recursion!");
    let _ = run(2,10);
}




fn run (_i: u32,max_n:u32) -> u32{


    debug!("run is running {} ",_i);
    debug!("fn name => {}",function_name!());

    if _i < max_n {

        run(_i,0);
    }

    1
    
}


