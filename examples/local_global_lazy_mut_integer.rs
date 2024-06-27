use once_cell::sync::Lazy;
// use std::sync::Mutex;

use log::debug;

// works
// const GLOBAL_DATA: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(0));

static  mut GLOBAL_DATA: Lazy<u8> = Lazy::new(|| 0);


fn main() {

    env_logger::init();
    
    // println!("{:#?}", *GLOBAL_DATA);
    debug!("{:#?}", *GLOBAL_DATA);
    second();
}


fn second(){
    debug!("{:#?}", *GLOBAL_DATA);


    *GLOBAL_DATA = 5;

    debug!("{:#?}", *GLOBAL_DATA);
}