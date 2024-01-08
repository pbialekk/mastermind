pub mod app;
pub mod judge;
pub mod run;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;

fn main() -> Result<()> {
    let result = run::run();

    result?;

    Ok(())
}
