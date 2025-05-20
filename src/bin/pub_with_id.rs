use std::{env, thread, time::Duration};
use prost::Message;
use zmq::Context;
#[path = "../packet.rs"]
pub mod packet;
fn main() {
    // Get publisher ID from command line
    let args: Vec<String> = env::args().collect();
    let publisher_id = args.get(1).unwrap_or(&"0".to_string()).clone();

    let ctx = Context::new();
    let pub_socket = ctx.socket(zmq::PUB).expect("Failed to create PUB");
    pub_socket
        .connect("tcp://localhost:6000")
        .expect("Failed to connect to XSUB");

    loop {
        let topic = format!("pub-id-{}", publisher_id.clone()); // Use publisher ID as topic
        let packet = packet::PacketHeader {
            id: publisher_id.parse().unwrap(),
            length: 1222,
            checksum: 0,
            version: 0,
            flags: 0,
        };

        let json = serde_json::to_string(&packet).unwrap();

        pub_socket
            .send_multipart(&[topic.as_bytes(), packet.encode_to_vec().as_slice()], zmq::DONTWAIT)
            .unwrap_or_else(|e| eprintln!("Send error: {}", e));

        println!("[Publisher {}] Sent: {}", publisher_id, json);
        thread::sleep(Duration::from_millis(1000));
    }
}
