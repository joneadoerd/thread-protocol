use std::thread;
use std::time::Duration;
use zmq::Context;

fn main() {
    let context = Context::new();
    let publisher = context.socket(zmq::PUB).expect("Failed to create PUB socket");

    let address = "tcp://*:5555";
    publisher
        .bind(address)
        .expect("Failed to bind PUB socket");

    println!("ZeroMQ publisher bound to {}", address);

    let mut counter = 0;
    loop {
        let msg = format!("topic Message {}", counter);
        match publisher.send(&msg, 0) {
            Ok(_) => println!("Sent: {}", msg),
            Err(e) => eprintln!("Failed to send: {}", e),
        }

        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
