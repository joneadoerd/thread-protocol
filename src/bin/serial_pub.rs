use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{Duration, sleep};
use tokio_serial::SerialPortBuilderExt;

#[path = "../packet.rs"]
pub mod packet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Open COM3 at 9600 baud
    let port = tokio_serial::new("COM4", 115200)
        .timeout(Duration::from_millis(100))
        .open_native_async()?;

    // Clone the port handle for split use
    let (mut reader, mut writer) = tokio::io::split(port);

    // Spawn receiver task
    let receiver = tokio::spawn(async move {
        loop {
            let mut buf = vec![0u8; 1024];
            match reader.read(&mut buf).await {
                Ok(n) => {
                    buf.truncate(n);
                    match packet::PacketHeader::decode(&*buf) {
                        Ok(packet) => {
                            println!("[Receiver] Received Packet: {:?}", packet);
                        }
                        Err(e) => {
                            eprintln!("[Receiver] Decode error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Receiver] Error: {}", e);
                    break;
                }
            }
        }
    });

    // Spawn sender task
    let sender = tokio::spawn(async move {
        let envelope = packet::PacketHeader {
            id: 1,
            length: 2,
            checksum: 3,
            version: 4,
            flags: 5,
        };

        loop {
            let mut buf = Vec::new();

            envelope.encode(&mut buf).unwrap();
            if let Err(e) = writer.write_all(&buf).await {
                eprintln!("[Sender] Error: {}", e);
                break;
            } else {
                println!("[Sender] Sent: {:?}", envelope);
                // println!("[Sender] Buffer: {:?}", buf);
            }
            sleep(Duration::from_millis(500)).await;
        }
    });

    // Wait for both tasks
    let _ = tokio::try_join!(receiver, sender)?;

    Ok(())
}
