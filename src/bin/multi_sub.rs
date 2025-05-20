use std::{sync::Arc, time::Duration};
use tokio::task;
use zmq::{Context, DONTWAIT};

async fn start_subscriber(ctx: Arc<Context>, topic: String, id: usize) {
    task::spawn_blocking(move || {
        let socket = ctx.socket(zmq::SUB).expect("Failed to create SUB");
        socket
            .connect("tcp://localhost:7000")
            .expect("Failed to connect");
        socket
            .set_subscribe(topic.as_bytes())
            .expect("Subscribe failed");

        println!("[Subscriber {}] Subscribed to '{}'", id, topic);

        loop {
            match socket.recv_multipart(DONTWAIT) {
                Ok(msg_parts) => {
                    if msg_parts.len() == 2 {
                        let topic = String::from_utf8_lossy(&msg_parts[0]);
                        let message = String::from_utf8_lossy(&msg_parts[1]);
                        println!("Received [{}]: {}", topic, message);
                    }
                }
                Err(_) => {
                    // eprintln!("Receive error: {}", e);
                }
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    });
}

#[tokio::main]
async fn main() {
    let ctx = Arc::new(Context::new());

    let sub_task = start_subscriber(Arc::clone(&ctx), "pub-id-1".to_string(), 1);

    let sub_task2 = start_subscriber(Arc::clone(&ctx), "pub-id-2".to_string(), 2);

    sub_task.await;
    sub_task2.await;
}
