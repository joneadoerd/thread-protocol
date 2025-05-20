// src/main.rs
mod communication;

use communication::serial::SerialComm;
use communication::strategy::CommunicationStrategy;

use crossbeam_channel::unbounded;
use std::sync::Arc;
use std::thread;
use zmq::Context;

use tokio::task;

async fn run_zmq_subscriber(ctx: Arc<Context>, address: String, topic: String, id: usize) {
    task::spawn_blocking(move || {
        let socket = ctx.socket(zmq::SUB).expect("Failed to create SUB socket");
        socket.connect(&address).expect("Failed to connect");
        socket
            .set_subscribe(topic.as_bytes())
            .expect("Failed to subscribe");

        println!("[Subscriber {}] Connected to {}", id, address);

        loop {
            match socket.recv_string(0) {
                Ok(Ok(msg)) => println!("[Subscriber {}] Received: {}", id, msg),
                Ok(Err(e)) => eprintln!("[Subscriber {}] UTF-8 error: {:?}", id, e),
                Err(e) => eprintln!("[Subscriber {}] ZMQ error: {:?}", id, e),
            }
        }
    });
}

#[tokio::main]
async fn main() {
    let (tx, rx) = unbounded();
    let ctx = Arc::new(Context::new());
    // Choose one strategy
    let use_serial = true;

    if use_serial {
        let mut strategy = SerialComm::new("com4".to_string(), 9600);
        strategy.start(tx.clone()).unwrap();
    } else {
        // let context = Arc::new(Context::new());
        // let mut strategy = ZmqComm::new(
        //     Arc::clone(&context),
        //     "tcp://localhost:5555".to_string(),
        //     "".to_string(),
        // );
        // strategy.start(tx.clone()).unwrap();

        // ZmqComm::new(context, "tcp://localhost:5555".to_string(), "".to_string())
        //     .start(tx.clone())
        //     .unwrap();
        let sub1 = run_zmq_subscriber(
            Arc::clone(&ctx),
            "tcp://localhost:5555".to_string(),
            "topic1".to_string(),
            1,
        );

        let sub2 = run_zmq_subscriber(
            Arc::clone(&ctx),
            "tcp://localhost:5555".to_string(),
            "topic2".to_string(),
            2,
        );
        tokio::join!(sub1, sub2);
    }

    // Main thread processes messages
    thread::spawn(move || {
        for msg in rx {
            println!("Received: {}", msg);
        }
    });

    // Keep main alive
    loop {
        thread::park();
    }
}
