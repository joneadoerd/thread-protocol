use std::{thread, time::Duration};

use zmq::Context;

fn main() {
    let ctx = Context::new();

    let socket = ctx.socket(zmq::SUB).expect("Failed to create SUB");
    socket
        .connect("tcp://localhost:7000")
        .expect("Failed to connect");

    socket
        .monitor("inproc://xsub-monitors", zmq::SocketEvent::ALL as i32)
        .expect("monitor failed");

    let monitor_socket = ctx.socket(zmq::PAIR).unwrap();
    monitor_socket.connect("inproc://xsub-monitors").unwrap();

    loop {
        let event_msg = monitor_socket.recv_msg(0).unwrap();
        let event = u16::from_le_bytes([event_msg[0], event_msg[1]]);
        println!("Event: {:?} ", zmq::SocketEvent::from_raw(event));

        let addr_msg = monitor_socket.recv_msg(0).unwrap();
        let addr = String::from_utf8_lossy(&addr_msg).to_string();
        println!("Address: {}", addr);
        if event == zmq::SocketEvent::ACCEPTED as u16 {
            println!("📡 New PUB connected.");
        } else if event == zmq::SocketEvent::DISCONNECTED as u16 {
            println!("📡 PUB disconnected.");
        }

        thread::sleep(Duration::from_secs(1));
    }
}
