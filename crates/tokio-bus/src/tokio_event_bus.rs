use asn_core::asn_event::AsnEvent;
use asn_core_bus::{AsnBus, AsnBusRecvError, AsnTransmitter};
use asn_core_bus::{AsnBusSendError, AsnReceiver};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;

struct TokioTransmitter<E> {
    tx: Sender<E>,
}

struct TokioReceiver<E> {
    rx: Receiver<E>,
}

impl<E> AsnTransmitter<E> for TokioTransmitter<E> {
    fn send_message(&self, m: E) -> Result<(), AsnBusSendError> {
        let result = self.tx.send(m);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(AsnBusSendError::Closed),
        }
    }
}

impl<E> AsnReceiver<E> for TokioReceiver<E>
where
    E: Clone,
{
    fn get_message(&mut self) -> Result<E, AsnBusRecvError> {
        let result = self.rx.try_recv();
        match result {
            Ok(e) => Ok(e),
            Err(err) => match err {
                broadcast::error::TryRecvError::Empty => Err(AsnBusRecvError::Empty),
                broadcast::error::TryRecvError::Closed => Err(AsnBusRecvError::Closed),
                broadcast::error::TryRecvError::Lagged(n) => {
                    panic!("TryRecvError::Lagged {:?}", n)
                }
            },
        }
    }
}

pub struct TokioEventBus<E> {
    tx: Sender<E>,
}

impl<E> TokioEventBus<E>
where
    E: Clone,
{
    fn new() -> Self {
        let (tx, _) = broadcast::channel::<E>(16);
        TokioEventBus { tx }
    }
}

impl<E> AsnBus<E> for TokioEventBus<E>
where
    E: Clone,
{
    fn get_sender(&self) -> impl AsnTransmitter<E> {
        TokioTransmitter {
            tx: self.tx.clone(),
        }
    }

    fn get_receiver(&self) -> impl AsnReceiver<E> {
        TokioReceiver {
            rx: self.tx.subscribe(),
        }
    }
}

pub fn new_tokio_bus<E: Clone>() -> impl AsnBus<E> {
    TokioEventBus::new()
}
