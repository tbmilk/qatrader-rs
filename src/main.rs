use std::io::stdin;
use std::thread;
use std::collections::HashMap;
use toml::Value;
use websocket::{Message, OwnedMessage};
use log::{error, info, warn};
use actix::prelude::System;

use qatrade_rs::config::CONFIG;
use qatrade_rs::log4::init_log4;
use qatrade_rs::scheduler::Scheduler;
use actix::Actor;


fn main() {
    let sys = System::new("");
    init_log4("log/qatrader.log", &CONFIG.common.log_level);
    let mut scheduler = Scheduler::new();
    scheduler.start_trader_loop();
    if let Err(e) = scheduler.start_ws_loop() {
        error!("websocket {:?}", e);
        std::process::exit(1);
    }
    scheduler.start_mq_loop();
    scheduler.start();
    sys.run();
}