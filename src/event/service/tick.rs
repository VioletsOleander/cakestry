use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, TryRecvError};

use crate::event::{TickEvent, TickStopEvent};

pub struct TickEventService {
    interval: u64,
    sender: Sender<TickEvent>,
    receiver: Receiver<TickStopEvent>,
}

impl TickEventService {
    pub fn new(sender: Sender<TickEvent>, receiver: Receiver<TickStopEvent>) -> Self {
        Self {
            interval: 100, // 0.1s
            sender,
            receiver,
        }
    }

    pub fn run(&self) {
        let interval = Duration::from_millis(self.interval);

        let sender = self.sender.clone();
        let receiver = self.receiver.clone();

        thread::spawn(move || {
            let mut count = 0;

            loop {
                if sender.send(TickEvent::new(count)).is_err() {
                    break;
                }

                match receiver.try_recv() {
                    Ok(_) => {
                        tracing::debug!("tick stop event received, tick thread exit");
                        break;
                    }
                    Err(err) => {
                        if err == TryRecvError::Disconnected {
                            tracing::debug!(
                                "tick stop sender unexpectedly disconnected due to {}",
                                err
                            );
                            break;
                        }
                    }
                }

                thread::sleep(interval);
                count += 1;
            }
        });
    }
}
