// FROM HERE
// https://www.sitepoint.com/rust-global-variables/

use chrono::Utc;
use log::debug;

thread_local!(static GLOBAL_DATA: String = Utc::now().to_string());

fn main() {

    env_logger::init();
    
    GLOBAL_DATA.with(|text| {
        println!("{}", *text);
    });

    test_global_var();
}

fn test_global_var() {
    debug!("test_global_var");
    GLOBAL_DATA.with(|text| {
        println!("{}", *text);
    });
}
