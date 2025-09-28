use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn init(debug_mode: bool) {
    let mut builder = Builder::from_default_env();

    if debug_mode {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.filter_level(LevelFilter::Info);
    }

    builder.target(Target::Stdout).init();
}
