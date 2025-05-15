// src/communication/strategy.rs
use anyhow::Result;
use crossbeam_channel::Sender;

pub trait CommunicationStrategy: Send + 'static {
    fn start(&mut self, tx: Sender<String>) -> Result<()>;
}
