// src/main.rs
mod communication;

use communication::serial::SerialComm;
use communication::strategy::CommunicationStrategy;
use communication::zeromq::ZmqComm;

use crossbeam_channel::unbounded;
use std::sync::Arc;
use std::thread;
use zmq::Context;

fn main() {
    let (tx, rx) = unbounded();

    // Choose one strategy
    let use_serial = false;

    if use_serial {
        let mut strategy = SerialComm::new("/com6".to_string(), 9600);
        strategy.start(tx.clone()).unwrap();
    } else {
        let context = Arc::new(Context::new());
        let mut strategy = ZmqComm::new(
            Arc::clone(&context),
            "tcp://localhost:5555".to_string(),
            "".to_string(),
        );
        strategy.start(tx.clone()).unwrap();

        ZmqComm::new(context, "tcp://localhost:5555".to_string(), "".to_string())
            .start(tx.clone())
            .unwrap();
    }

    // Main thread processes messages
    thread::spawn(move || {
        for msg in rx {
            println!("Received: {}", msg);
        }
    });

    // Keep main alive
    loop {
        std::thread::park();
    }
}
