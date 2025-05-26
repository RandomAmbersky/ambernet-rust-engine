use crate::AsnLogLevel;
use log::LevelFilter;

pub fn convert(l: AsnLogLevel) -> LevelFilter {
    match l {
        AsnLogLevel::Off => LevelFilter::Off,
        AsnLogLevel::Error => LevelFilter::Error,
        AsnLogLevel::Warn => LevelFilter::Warn,
        AsnLogLevel::Info => LevelFilter::Info,
        AsnLogLevel::Debug => LevelFilter::Debug,
        AsnLogLevel::Trace => LevelFilter::Trace,
    }
}
