extern crate log;
extern crate env_logger;

use std::env;
use log::{info, debug, trace, warn, error};

fn main() {
    env_logger::init();
    trace!("A trace Hello, world! mygreatapp {}", env::var("MY_GREAT_CONFIG").unwrap());
    debug!("A debug Hello, world! mygreatapp {}", env::var("MY_GREAT_CONFIG").unwrap());
    info!("An info Hello, world! mygreatapp {}", env::var("MY_GREAT_CONFIG").unwrap());
    warn!("A warn Hello, world! mygreatapp {}", env::var("MY_GREAT_CONFIG").unwrap());
    error!("An error Hello, world! mygreatapp {}", env::var("MY_GREAT_CONFIG").unwrap());
}
