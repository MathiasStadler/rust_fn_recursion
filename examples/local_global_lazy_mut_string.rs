use once_cell::sync::Lazy;
use chrono::Utc;
use log::debug;

static GLOBAL_DATA: Lazy<String> = Lazy::new(||Utc::now().to_string());

fn main() {

    env_logger::init();
    
    debug!("{}", *GLOBAL_DATA);
}
