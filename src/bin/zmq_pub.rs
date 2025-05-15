use std::{thread, time::Duration};
use zmq::Context;

fn main() {
    let context = Context::new();
    let publisher = context.socket(zmq::PUB).expect("Failed to create PUB socket");

    let address = "tcp://*:5555";
    publisher.bind(address).expect("Failed to bind");

    println!("Publisher started on {}", address);

    let mut counter = 0;
    loop {
        let msg1 = format!("topic1 Hello from topic1 #{}", counter);
        let msg2 = format!("topic2 Hello from topic2 #{}", counter);

        publisher.send(&msg1, 0).unwrap();
        publisher.send(&msg2, 0).unwrap();

        println!("Sent message #{}", counter);
        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
