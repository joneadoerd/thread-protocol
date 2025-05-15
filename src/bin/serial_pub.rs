use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    let port_name = "COM3"; // Change as needed
    let baud_rate = 9600;

    let mut port = match serialport::new(port_name, baud_rate)
        .timeout(Duration::from_secs(2))
        .open()
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to open serial port: {}", e);
            return;
        }
    };

    println!("Writing to serial port...");
    let mut counter = 0;

    loop {
        let msg = format!("Serial Message {}\n", counter);
        if let Err(e) = port.write(msg.as_bytes()) {
            eprintln!("Failed to write to serial: {}", e);
        } else {
            println!("Sent: {}", msg.trim());
        }

        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
