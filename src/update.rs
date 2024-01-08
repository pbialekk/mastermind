use crate::app::App;
use anyhow::Result;
use crossterm::event::{self, Event::Key};

pub fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    event::KeyCode::Char('q') => {
                        app.should_quit = true;
                    }
                    event::KeyCode::Right => {
                        app.next_tab();
                        app.reset_option();
                    }
                    event::KeyCode::Left => {
                        app.previous_tab();
                        app.reset_option();
                    }
                    event::KeyCode::Up => {
                        app.next_option();
                        app.choose_option();
                    }
                    event::KeyCode::Down => {
                        app.previous_option();
                        app.choose_option();
                    }
                    event::KeyCode::Enter => {
                        app.submit();
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
