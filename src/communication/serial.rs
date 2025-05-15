// src/communication/serial.rs
use super::strategy::CommunicationStrategy;
use anyhow::Result;
use crossbeam_channel::Sender;
use std::time::Duration;
use std::thread;
use std::io::{BufRead, BufReader};

pub struct SerialComm {
    port_name: String,
    baud_rate: u32,
}

impl SerialComm {
    pub fn new(port_name: String, baud_rate: u32) -> Self {
        Self { port_name, baud_rate }
    }
}

impl CommunicationStrategy for SerialComm {
    fn start(&mut self, tx: Sender<String>) -> Result<()> {
        let port_name = self.port_name.clone();
        let baud_rate = self.baud_rate;

        thread::spawn(move || {
            let port = serialport::new(port_name, baud_rate)
                .timeout(Duration::from_millis(1000))
                .open();

            match port {
                Ok(p) => {
                    let reader = BufReader::new(p);
                    for line in reader.lines() {
                        if let Ok(data) = line {
                            let _ = tx.send(data);
                        }
                    }
                }
                Err(e) => eprintln!("Serial port error: {:?}", e),
            }
        });

        Ok(())
    }
}
