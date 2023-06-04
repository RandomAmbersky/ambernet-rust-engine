mod log_level;

use log::LevelFilter;
pub use log_level::AsnLogLevel;

fn _init_log(global_log_filter: LevelFilter) {
    let mut builder = fern::Dispatch::new();
    let level_formatter;
    #[cfg(target_arch = "wasm32")]
    {
        level_formatter = |level| level;
        builder = builder.chain(fern::Output::call(console_log::log));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use fern::colors::{Color, ColoredLevelConfig};
        let colors = ColoredLevelConfig::new()
            .info(Color::Blue)
            .debug(Color::Green);
        level_formatter = move |level| colors.color(level);
        builder = builder.chain(std::io::stdout());
    }
    builder
        .level(global_log_filter)
        .level_for(module_path!(), log::LevelFilter::Debug)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}:{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                level_formatter(record.level()),
                record.target(),
                record.line().unwrap_or_default(),
                message
            ))
        })
        .apply()
        .unwrap();
}

// const GLOBAL_LOG_FILTER: log::LevelFilter = log::LevelFilter::Info;

pub fn init_log(l: AsnLogLevel) {
    let log_level = match l {
        AsnLogLevel::Off => log::LevelFilter::Off,
        AsnLogLevel::Error => log::LevelFilter::Error,
        AsnLogLevel::Warn => log::LevelFilter::Warn,
        AsnLogLevel::Info => log::LevelFilter::Info,
        AsnLogLevel::Debug => log::LevelFilter::Debug,
        AsnLogLevel::Trace => log::LevelFilter::Trace,
    };
    _init_log(log_level)
}
