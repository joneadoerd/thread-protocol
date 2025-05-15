use std::sync::Arc;
use tokio::task;
use zmq::Context;

async fn start_subscriber(ctx: Arc<Context>, topic: String, id: usize) {
    task::spawn_blocking(move || {
        let socket = ctx.socket(zmq::SUB).expect("Failed to create SUB");
        socket.connect("tcp://localhost:7000").expect("Failed to connect");
        socket.set_subscribe(topic.as_bytes()).expect("Subscribe failed");

        println!("[Subscriber {}] Subscribed to '{}'", id, topic);

        loop {
            match socket.recv_string(0) {
                Ok(Ok(msg)) => {
                    // Expected message format: "topic id|payload"
                    let parts: Vec<&str> = msg.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        let topic = parts[0];
                        let data = parts[1];

                        let data_parts: Vec<&str> = data.splitn(2, '|').collect();
                        if data_parts.len() == 2 {
                            let pub_id = data_parts[0];
                            let payload = data_parts[1];
                            println!(
                                "[Subscriber {}] Topic: {}, From Publisher {} => {}",
                                id, topic, pub_id, payload
                            );
                        }
                    }
                }
                Ok(Err(e)) => eprintln!("[Subscriber {}] Invalid UTF-8: {:?}", id, e),
                Err(e) => eprintln!("[Subscriber {}] ZMQ error: {:?}", id, e),
            }
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
