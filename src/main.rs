use clap::Parser;
use color_eyre::Result;
use log::debug;
use picklist::{App, Config};
mod tui;

fn setup_logging() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();
}

fn main() -> Result<()> {
    tui::install_error_hooks()?;

    setup_logging();
    let config = Config::parse();
    debug!("{:?}", config);

    let terminal = tui::init_terminal()?;
    App::new(config).run(terminal)?;
    tui::restore_terminal()?;

    Ok(())
}
