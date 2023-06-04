#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum AsnLogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
