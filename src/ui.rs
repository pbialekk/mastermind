use crate::app::{Answer, App, Shape};
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, *},
};

pub fn gen_shape(shape: &Shape, answer: Option<Answer>) -> impl Widget + '_ {
    Canvas::default()
        .block(
            Block::default()
                .title(format!(
                    "{}",
                    if let Some(answer) = answer {
                        answer.to_string()
                    } else {
                        "Generated shape".to_string()
                    }
                ))
                .borders(Borders::ALL),
        )
        .x_bounds([10., 210.])
        .y_bounds([10., 110.])
        .paint(shape.get_shape_closures())
}

pub fn render(app: &App, f: &mut Frame) {
    let area = f.size();
    let horizontal_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(45), Constraint::Min(0)].as_ref())
        .split(area);

    let vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(14),
                Constraint::Length(10),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(horizontal_split[0]);

    let block = Block::default();

    f.render_widget(block, area);
    let titles = app.tab_titles().iter().map(|t| Line::from(*t)).collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Property"))
        .select(app.current_tab as usize)
        .highlight_style(Style::default().bold().on_light_blue());

    f.render_widget(tabs, vertical_split[0]);

    let titles = app.option_titles().iter().map(|t| Line::from(*t)).collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Option"))
        .select(usize::from(app.current_tab_option))
        .highlight_style(Style::default().bold().on_light_blue());

    f.render_widget(tabs, vertical_split[1]);

    // print currently selected shape

    if app.won_game {
        let text = Paragraph::new("You won the game!")
            .block(Block::default().borders(Borders::ALL).title("Game over"))
            .alignment(Alignment::Center);
        f.render_widget(text, vertical_split[2]);
    } else {
        f.render_widget(gen_shape(&app.current_shape, None), vertical_split[2]);
    }

    // render text
    let text = Paragraph::new(app.guide())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("A quick guide"),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(text, vertical_split[3]);

    // print already submitted shapes

    let num_rows = (horizontal_split[1].height / 10) as usize;
    let num_cols = (horizontal_split[1].width / 32) as usize;

    if num_rows == 0 || num_cols == 0 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(100 / num_cols as u16);
            num_cols
        ])
        .split(horizontal_split[1]);

    let mut grid = Vec::new();

    for i in 0..num_cols {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(100 / num_rows as u16);
                num_rows
            ])
            .split(chunks[i]);
        grid.push(chunks);
    }

    let mut row = 0;
    let mut col = 0;
    for (shape, answer) in app.previous_shapes.iter().rev().take(num_rows * num_cols) {
        let shape = gen_shape(shape, Some(*answer));
        f.render_widget(shape, grid[col][row]);
        row += 1;
        if row == num_rows {
            row = 0;
            col += 1;
        }
    }
}
