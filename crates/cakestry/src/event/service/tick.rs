use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, TryRecvError, bounded};

use crate::event::{TickEvent, TickStopEvent};

#[derive(Clone)]
pub struct TickEventService {
    interval: u64,
    tick_tx: Sender<TickEvent>,
    tickstop_tx: Sender<TickStopEvent>,
    tickstop_rx: Receiver<TickStopEvent>,
}

impl TickEventService {
    pub fn new(tick_tx: Sender<TickEvent>) -> Self {
        let (tickstop_tx, tickstop_rx) = bounded(1);

        Self {
            interval: 100, // 0.1s
            tick_tx,
            tickstop_tx,
            tickstop_rx,
        }
    }

    pub fn run(&self) {
        let interval = Duration::from_millis(self.interval);

        let tick_tx = self.tick_tx.clone();
        let tickstop_rx = self.tickstop_rx.clone();

        thread::spawn(move || {
            let mut count = 0;

            loop {
                if tick_tx.send(TickEvent::new(count)).is_err() {
                    break;
                }

                match tickstop_rx.try_recv() {
                    Ok(_) => {
                        tracing::debug!("tick stop event received, tick thread exit");
                        break;
                    }
                    Err(err) if err == TryRecvError::Disconnected => {
                        tracing::debug!(
                            "tick stop sender unexpectedly disconnected due to {}",
                            err
                        );
                        break;
                    }
                    _ => (),
                }

                thread::sleep(interval);
                count += 1;
            }
        });
    }

    pub fn stop(&self) {
        // If thread blocking `send` is used here, deadlock will possibly happen when the channel is
        // zero capacity: the main thread waiting for the tick thread to receive TickStopEvent, the
        // tick thread wait for the main thread to receive TickEvent.
        self.tickstop_tx
            .try_send(TickStopEvent)
            .unwrap_or_else(|err| tracing::debug!("failed to send tick stop event due to {}", err));
    }
}
