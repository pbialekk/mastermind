use crate::app::App;
use crate::tui::Tui;
use crate::update::update;
use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

pub fn run() -> Result<()> {
    let mut app = App::default();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);

    tui.enter()?;

    while !app.should_quit {
        update(&mut app)?;

        tui.draw(&mut app)?;
    }

    tui.exit()?;
    Ok(())
}
