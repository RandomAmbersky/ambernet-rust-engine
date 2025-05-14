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

pub trait AsnTransmitter<M> {
    fn send_message(&self, m: M) -> Result<(), AsnBusSendError>;
}

pub trait AsnReceiver<M> {
    fn get_message(&mut self) -> Result<M, AsnBusRecvError>;
}

pub trait AsnBus<M> {
    fn get_sender(&self) -> impl AsnTransmitter<M>;
    fn get_receiver(&self) -> impl AsnReceiver<M>;
}
