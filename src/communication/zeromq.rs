// src/communication/zeromq.rs
use super::strategy::CommunicationStrategy;
use anyhow::Result;
use crossbeam_channel::Sender;
use std::sync::Arc;
use std::thread;
use zmq::Context;

pub struct ZmqComm {
    context: Arc<Context>,
    address: String,
    topic: String,
}

impl ZmqComm {
    pub fn new(context: Arc<Context>, address: String, topic: String) -> Self {
        Self { context, address, topic }
    }
}

impl CommunicationStrategy for ZmqComm {
    fn start(&mut self, tx: Sender<String>) -> Result<()> {
        let address = self.address.clone();
        let topic = self.topic.clone();
        let ctx = Arc::clone(&self.context);

        thread::spawn(move || {
            let subscriber = ctx.socket(zmq::SUB).expect("Failed to create SUB socket");
            subscriber.connect(&address).expect("Failed to connect");
            subscriber.set_subscribe(topic.as_bytes()).expect("Failed to subscribe");

            loop {
                match subscriber.recv_string(0) {
                    Ok(Ok(msg)) => {
                        let _ = tx.send(msg);
                    }
                    Ok(Err(e)) => eprintln!("UTF-8 error: {:?}", e),
                    Err(e) => eprintln!("ZMQ error: {:?}", e),
                }
            }
        });

        Ok(())
    }
}
