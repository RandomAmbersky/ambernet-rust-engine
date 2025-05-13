#[derive(Debug)]
pub enum AsnBusSendError {
    Full,
    Disconnected,
    Closed,
}

#[derive(Debug)]
pub enum AsnBusRecvError {
    Empty,
    Closed,
}

pub trait AsnTransmitter<T> {
    fn send_message(&self, m: T) -> Result<(), AsnBusSendError>;
}

pub trait AsnReceiver<T> {
    fn get_message(&mut self) -> Result<T, AsnBusRecvError>;
}

pub trait AsnBus<E> {
    fn get_sender(&self) -> impl AsnTransmitter<E>;
    fn get_receiver(&self) -> impl AsnReceiver<E>;
}
