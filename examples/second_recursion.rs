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


    println!("Hello, second recursion!");

    info!("Hello, second recursion!");
    let _ = run(2);
}


async fn run (i: u32) -> u32{

    
    let fn_name = function_name!()
              .rsplit_once(':')
              .expect("could not parse function name")
              .1;

              println!("{}", fn_name);
    
    debug!("start => async_fibonacci");
    async_fibonacci(i).await
}

async fn async_fibonacci(i: u32) -> u32 {
    if i == 0 || i == 1 {
        i
    } else {
        Box::pin(async_fibonacci(i - 1)).await + Box::pin(async_fibonacci(i - 2)).await
    }
}
