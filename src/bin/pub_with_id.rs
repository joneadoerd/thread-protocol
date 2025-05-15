use std::{env, thread, time::Duration};
use zmq::Context;

fn main() {
    // Get publisher ID from command line
    let args: Vec<String> = env::args().collect();
    let publisher_id = args.get(1).unwrap_or(&"0".to_string()).clone();

    let ctx = Context::new();
    let pub_socket = ctx.socket(zmq::PUB).expect("Failed to create PUB");
    pub_socket.connect("tcp://localhost:6000").expect("Failed to connect to XSUB");

    let mut counter = 0;
    loop {
        let topic = format!("pub-id-{}", publisher_id.clone()); // Use publisher ID as topic
        let payload = format!("{}|{}", publisher_id.clone(), counter); // Embed ID in message
        let msg = format!("{} {}", topic, payload);
        pub_socket.send(&msg, 0).unwrap();

        println!("[Publisher {}] Sent: {}", publisher_id, msg);
        counter += 1;
        thread::sleep(Duration::from_millis(1000));
    }
}
