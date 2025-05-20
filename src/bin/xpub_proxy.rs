use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use zmq::{Context, SocketType};

fn main() {
    let ctx = Context::new();

    let xsub = ctx.socket(SocketType::XSUB).expect("XSUB failed");
    let xpub = ctx.socket(SocketType::XPUB).expect("XPUB failed");

    xsub.bind("tcp://*:6000").expect("XSUB bind failed"); // PUBs
    xpub.bind("tcp://*:7000").expect("XPUB bind failed"); // SUBs

    xpub.set_xpub_verbose(true)
        .expect("Enable XPUB verbose failed");

    println!("Proxy running. PUBs connect to :6000, SUBs connect to :7000");

    let topic_subs: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let pub_count = Arc::new(Mutex::new(0));

    // Start monitor for XSUB to track PUB connections
    let monitor_addr = "inproc://xsub-monitor";
    xsub.monitor(monitor_addr, zmq::SocketEvent::ALL as i32)
        .expect("Failed to start monitor");

    {
        let ctx = ctx.clone();
        let pub_count = Arc::clone(&pub_count);
        let monitor_addr = monitor_addr.to_string();
        thread::spawn(move || {
            let monitor_socket = ctx.socket(SocketType::PAIR).unwrap();
            monitor_socket.connect(&monitor_addr).unwrap();

            loop {
                // Each monitor event is a pair of frames: [event, address]
                let event_msg = match monitor_socket.recv_msg(0) {
                    Ok(msg) => msg,
                    Err(_) => break,
                };
                let _addr_msg = match monitor_socket.recv_msg(0) {
                    Ok(msg) => msg,
                    Err(_) => break,
                };
                if event_msg.len() >= 6 {
                    let event = u16::from_le_bytes([event_msg[0], event_msg[1]]);
                    // let value = u32::from_le_bytes([event_msg[2], event_msg[3], event_msg[4], event_msg[5]]);
                    match zmq::SocketEvent::from_raw(event) {
                        zmq::SocketEvent::ACCEPTED => {
                            let mut count = pub_count.lock().unwrap();
                            *count += 1;
                            println!("📡 New PUB connected. Total: {}", *count);
                        }
                        zmq::SocketEvent::DISCONNECTED => {
                            let mut count = pub_count.lock().unwrap();
                            if *count > 0 {
                                *count -= 1;
                            }
                            println!("🔌 PUB disconnected. Remaining: {}", *count);
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    // Spawn periodic stats printer
    {
        let topic_subs = Arc::clone(&topic_subs);
        let pub_count = Arc::clone(&pub_count);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(5));
                let pub_count = *pub_count.lock().unwrap();
                let subs = topic_subs.lock().unwrap();
                println!("===================");
                println!("📈 Current Stats:");
                println!("  PUBs: {}", pub_count);
                for (topic, count) in subs.iter() {
                    println!("  SUBs on '{}': {}", topic, count);
                }
                println!("===================");
            }
        });
    }

    // Main forwarding loop
    loop {
        // Handle SUB subscriptions
        if let Ok(msg) = xpub.recv_msg(zmq::DONTWAIT) {
            let data = msg.as_ref();
            if !data.is_empty() {
                let is_sub = data[0] == 1;
                let topic = String::from_utf8_lossy(&data[1..]).to_string();

                let mut subs = topic_subs.lock().unwrap();
                let count = subs.entry(topic.clone()).or_insert(0);
                if is_sub {
                    *count += 1;
                    println!("➕ SUBSCRIBED to '{}'. Total: {}", topic, count);
                } else if *count > 0 {
                    *count -= 1;
                    println!("➖ UNSUBSCRIBED from '{}'. Remaining: {}", topic, count);
                }
            }

            // Forward sub control messages to XSUB
            xsub.send(msg, 0).expect("Forward to XSUB failed");
        }

        loop {
            let mut msg_parts = vec![];
            // Receive all parts of a multipart message
            loop {
                match xsub.recv_msg(zmq::DONTWAIT) {
                    Ok(part) => {
                        let more = xsub.get_rcvmore().unwrap();
                        msg_parts.push(part);
                        if !more {
                            break;
                        }
                    }
                    Err(e) if e == zmq::Error::EAGAIN => break, // No more messages now
                    Err(e) => {
                        eprintln!("Error receiving from XSUB: {}", e);
                        break;
                    }
                }
            }

            if !msg_parts.is_empty() {
                let parts_len = msg_parts.len();
                for (i, part) in msg_parts.into_iter().enumerate() {
                    let flags = if i == parts_len - 1 {
                        0
                    } else {
                        zmq::SNDMORE
                    };
                    xpub.send(part, flags).expect("Send to XPUB failed");
                }
            }

            // Also check for subscription control messages
            if let Ok(msg) = xpub.recv_msg(zmq::DONTWAIT) {
                let data = msg.as_ref();
                if !data.is_empty() {
                    let is_sub = data[0] == 1;
                    let topic = String::from_utf8_lossy(&data[1..]).to_string();

                    let mut subs = topic_subs.lock().unwrap();
                    let count = subs.entry(topic.clone()).or_insert(0);
                    if is_sub {
                        *count += 1;
                        println!("➕ SUBSCRIBED to '{}'. Total: {}", topic, count);
                    } else if *count > 0 {
                        *count -= 1;
                        println!("➖ UNSUBSCRIBED from '{}'. Remaining: {}", topic, count);
                    }
                }

                // Forward sub control message to XSUB
                xsub.send(msg, 0).expect("Forward to XSUB failed");
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
}
