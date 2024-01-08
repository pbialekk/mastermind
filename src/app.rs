use ratatui::widgets::canvas::Context;

use crate::judge::Judge;

#[derive(Clone, Copy, Debug, Default)]
pub enum Menu {
    #[default]
    Figure,
    Color,
    DotPosition,
    DotColor,
}
#[derive(Clone, Copy, Debug)]
pub enum MenuOption {
    Figure(Figure),
    Color(Color),
    DotPosition(DotPosition),
    DotColor(Color),
}
impl From<MenuOption> for usize {
    fn from(menu_option: MenuOption) -> Self {
        match menu_option {
            MenuOption::Figure(figure) => figure as usize,
            MenuOption::Color(color) => color as usize,
            MenuOption::DotPosition(pattern) => pattern as usize,
            MenuOption::DotColor(color) => color as usize,
        }
    }
}

impl Menu {
    pub fn next_tab(&mut self) {
        match self {
            Menu::Figure => *self = Menu::Color,
            Menu::Color => *self = Menu::DotPosition,
            Menu::DotPosition => *self = Menu::DotColor,
            Menu::DotColor => *self = Menu::DotColor,
        }
    }
    pub fn previous_tab(&mut self) {
        match self {
            Menu::Figure => *self = Menu::Figure,
            Menu::Color => *self = Menu::Figure,
            Menu::DotPosition => *self = Menu::Color,
            Menu::DotColor => *self = Menu::DotPosition,
        }
    }
}

impl MenuOption {
    pub fn next_option(&mut self) {
        match self {
            MenuOption::Figure(figure) => figure.next_option(),
            MenuOption::Color(color) => color.next_option(),
            MenuOption::DotPosition(pattern) => pattern.next_option(),
            MenuOption::DotColor(color) => color.next_option(),
        }
    }
    pub fn previous_option(&mut self) {
        match self {
            MenuOption::Figure(figure) => figure.previous_option(),
            MenuOption::Color(color) => color.previous_option(),
            MenuOption::DotPosition(pattern) => pattern.previous_option(),
            MenuOption::DotColor(color) => color.previous_option(),
        }
    }
}
impl Default for MenuOption {
    fn default() -> Self {
        MenuOption::Figure(Figure::default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Figure {
    #[default]
    Circle,
    Rectangle,
    Triangle,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Color {
    #[default]
    Red,
    Green,
    Blue,
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum DotPosition {
    #[default]
    Left,
    Middle,
    Right,
}

impl Figure {
    pub fn next_option(&mut self) {
        match self {
            Figure::Circle => *self = Figure::Rectangle,
            Figure::Rectangle => *self = Figure::Triangle,
            Figure::Triangle => *self = Figure::Triangle,
        }
    }
    pub fn previous_option(&mut self) {
        match self {
            Figure::Circle => *self = Figure::Circle,
            Figure::Rectangle => *self = Figure::Circle,
            Figure::Triangle => *self = Figure::Rectangle,
        }
    }
    pub fn random() -> Self {
        match rand::random::<u8>() % 3 {
            0 => Figure::Circle,
            1 => Figure::Rectangle,
            2 => Figure::Triangle,
            _ => panic!("Impossible"),
        }
    }
}
impl Color {
    pub fn next_option(&mut self) {
        match self {
            Color::Red => *self = Color::Green,
            Color::Green => *self = Color::Blue,
            Color::Blue => *self = Color::Blue,
        }
    }
    pub fn previous_option(&mut self) {
        match self {
            Color::Red => *self = Color::Red,
            Color::Green => *self = Color::Red,
            Color::Blue => *self = Color::Green,
        }
    }
    pub fn get_color(&self) -> ratatui::style::Color {
        match self {
            Color::Red => ratatui::style::Color::Red,
            Color::Green => ratatui::style::Color::Green,
            Color::Blue => ratatui::style::Color::Blue,
        }
    }
    pub fn random() -> Self {
        match rand::random::<u8>() % 3 {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            _ => panic!("Impossible"),
        }
    }
}
impl DotPosition {
    pub fn next_option(&mut self) {
        match self {
            DotPosition::Left => *self = DotPosition::Middle,
            DotPosition::Middle => *self = DotPosition::Right,
            DotPosition::Right => *self = DotPosition::Right,
        }
    }
    pub fn previous_option(&mut self) {
        match self {
            DotPosition::Left => *self = DotPosition::Left,
            DotPosition::Middle => *self = DotPosition::Left,
            DotPosition::Right => *self = DotPosition::Middle,
        }
    }
    pub fn random() -> Self {
        match rand::random::<u8>() % 3 {
            0 => DotPosition::Left,
            1 => DotPosition::Middle,
            2 => DotPosition::Right,
            _ => panic!("Impossible"),
        }
    }
}

impl std::fmt::Display for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Figure::Circle => write!(f, "Circle"),
            Figure::Rectangle => write!(f, "Rectangle"),
            Figure::Triangle => write!(f, "Triangle"),
        }
    }
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}
impl std::fmt::Display for DotPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotPosition::Left => write!(f, "Left"),
            DotPosition::Middle => write!(f, "Middle"),
            DotPosition::Right => write!(f, "Right"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Answer {
    pub correct: u8,
}
impl Answer {
    pub fn add_correct(&mut self) {
        self.correct += 1;
    }
    pub fn set_correct(&mut self, correct: u8) {
        self.correct = correct;
    }
}
impl std::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "✔ ".repeat(self.correct as usize))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Shape {
    pub figure: Figure,
    pub color: Color,
    pub dot_position: DotPosition,
    pub dot_color: Color,
}

impl Shape {
    pub fn random() -> Self {
        Self {
            figure: Figure::random(),
            color: Color::random(),
            dot_position: DotPosition::random(),
            dot_color: Color::random(),
        }
    }
    pub fn get_shape_closures(&self) -> impl Fn(&mut Context<'_>) + '_ {
        use ratatui::widgets::canvas::{Circle, Rectangle};
        move |ctx| {
            match self.dot_position {
                DotPosition::Left => {
                    ctx.draw(&Circle {
                        x: 30.,
                        y: 60.,
                        radius: 12.,
                        color: self.dot_color.get_color(),
                    });
                }
                DotPosition::Middle => {
                    ctx.draw(&Circle {
                        x: 110.,
                        y: 60.,
                        radius: 12.,
                        color: self.dot_color.get_color(),
                    });
                }
                DotPosition::Right => {
                    ctx.draw(&Circle {
                        x: 190.,
                        y: 60.,
                        radius: 12.,
                        color: self.dot_color.get_color(),
                    });
                }
            };
            ctx.layer();
            match self.figure {
                Figure::Circle => ctx.draw(&Circle {
                    x: 110.,
                    y: 60.,
                    radius: 50.,
                    color: self.color.get_color(),
                }),
                Figure::Rectangle => ctx.draw(&Rectangle {
                    x: 55.,
                    y: 30.,
                    width: 110.,
                    height: 60.,
                    color: self.color.get_color(),
                }),
                Figure::Triangle => {
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: 55.,
                        y1: 30.,
                        x2: 55. + 110.,
                        y2: 30.,
                        color: self.color.get_color(),
                    });
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: 55.,
                        y1: 30.,
                        x2: 55. + 110. / 2.,
                        y2: 30. + 60. / 0.9,
                        color: self.color.get_color(),
                    });
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: 55. + 110. / 2.,
                        y1: 30. + 60. / 0.9,
                        x2: 55. + 110.,
                        y2: 30.,
                        color: self.color.get_color(),
                    });
                }
            };
        }
    }
}

#[derive(Clone, Debug)]
pub struct App {
    pub should_quit: bool,
    pub won_game: bool,
    pub current_shape: Shape,
    pub current_tab: Menu,
    pub current_tab_option: MenuOption,
    pub previous_shapes: Vec<(Shape, Answer)>,
    judge: Judge,
}

impl App {
    pub fn next_tab(&mut self) {
        self.current_tab.next_tab();
    }
    pub fn previous_tab(&mut self) {
        self.current_tab.previous_tab();
    }
    pub fn next_option(&mut self) {
        self.current_tab_option.next_option();
    }
    pub fn previous_option(&mut self) {
        self.current_tab_option.previous_option();
    }
    pub fn reset_option(&mut self) {
        match self.current_tab {
            Menu::Figure => self.current_tab_option = MenuOption::Figure(self.current_shape.figure),
            Menu::Color => self.current_tab_option = MenuOption::Color(self.current_shape.color),
            Menu::DotPosition => {
                self.current_tab_option = MenuOption::DotPosition(self.current_shape.dot_position)
            }
            Menu::DotColor => {
                self.current_tab_option = MenuOption::DotColor(self.current_shape.dot_color)
            }
        }
    }
    pub fn choose_option(&mut self) {
        match self.current_tab {
            Menu::Figure => {
                if let MenuOption::Figure(figure) = self.current_tab_option {
                    self.current_shape.figure = figure;
                } else {
                    panic!("MenuOption::Figure expected");
                }
            }
            Menu::Color => {
                if let MenuOption::Color(color) = self.current_tab_option {
                    self.current_shape.color = color;
                } else {
                    panic!("MenuOption::Color expected");
                }
            }
            Menu::DotPosition => {
                if let MenuOption::DotPosition(pattern) = self.current_tab_option {
                    self.current_shape.dot_position = pattern;
                } else {
                    panic!("MenuOption::Pattern expected");
                }
            }
            Menu::DotColor => {
                if let MenuOption::DotColor(color) = self.current_tab_option {
                    self.current_shape.dot_color = color;
                } else {
                    panic!("MenuOption::Color expected");
                }
            }
        }
    }
    pub fn submit(&mut self) {
        if self.won_game {
            return;
        }
        let answer = self.judge.judge(&self.current_shape);
        self.previous_shapes.push((self.current_shape, answer));
        if answer.correct == 4 {
            self.won_game = true;
        }
    }
    pub fn tab_titles(&self) -> Vec<&str> {
        vec!["Figure", "Color", "Dot Position", "Dot Color"]
    }
    pub fn option_titles(&self) -> Vec<&str> {
        match self.current_tab {
            Menu::Figure => vec!["Circle", "Square", "Triangle"],
            Menu::Color => vec!["Red", "Green", "Blue"],
            Menu::DotPosition => vec!["Left", "Middle", "Right"],
            Menu::DotColor => vec!["Red", "Green", "Blue"],
        }
    }
    pub fn guide(&self) -> String {
        "Your goal is to guess the hidden shape using as little queries as possible. Use the arrow keys to select a property of the shape / switch between properties. Press Enter to submit the shape. In a response, you will be given the number of properties which match between your shape, and the hidden one. Press Q to quit the game.".to_string()
    }
}
impl Default for App {
    fn default() -> Self {
        use std::env;
        Self {
            should_quit: false,
            won_game: false,
            current_shape: Shape::default(),
            current_tab: Menu::default(),
            current_tab_option: MenuOption::default(),
            previous_shapes: Vec::new(),
            judge: Judge::new(),
        }
    }
}
