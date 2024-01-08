use crate::app::{Answer, /* Color, DotPosition, Figure, */ Shape};

#[derive(Clone, Debug)]
pub struct Judge {
    shape: Shape,
    judge_history: Vec<(Shape, Answer)>,
    // available: Vec<(Shape, Answer)>,
    hardcore: bool,
}

impl Judge {
    pub fn new(hardcore: bool) -> Self {
        // let mut available: Vec<(Shape, Answer)> = Vec::new();
        // const FIGURES: [Figure; 3] = [Figure::Circle, Figure::Rectangle, Figure::Triangle];
        // const COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];
        // const PATTERNS: [DotPosition; 3] =
        //     [DotPosition::Left, DotPosition::Middle, DotPosition::Right];
        // const ANSWERS: [Answer; 4] = [
        //     Answer { correct: 0 },
        //     Answer { correct: 1 },
        //     Answer { correct: 2 },
        //     Answer { correct: 3 },
        // ];
        // for figure in FIGURES.iter() {
        //     for color in COLORS.iter() {
        //         for pattern in PATTERNS.iter() {
        //             for dot_color in COLORS.iter() {
        //                 for answer in ANSWERS.iter() {
        //                     let shape = Shape {
        //                         figure: *figure,
        //                         color: *color,
        //                         dot_position: *pattern,
        //                         dot_color: *dot_color,
        //                     };
        //                     available.push((shape, *answer));
        //                 }
        //             }
        //         }
        //     }
        // }
        Self {
            shape: Shape::random(),
            judge_history: Vec::new(),
            // available,
            hardcore,
        }
    }
    // fn intersection(&self, first_shape: &Shape, second_shape: &Shape) -> Answer {
    //     let mut intersection = Answer::default();
    //     if first_shape.figure != second_shape.figure {
    //         intersection.add_correct();
    //     }
    //     if first_shape.color != second_shape.color {
    //         intersection.add_correct();
    //     }
    //     if first_shape.dot_position != second_shape.dot_position {
    //         intersection.add_correct();
    //     }
    //     intersection
    // }
    // fn erase_unavailable(&self, shape: &Shape, answer: &Answer) -> Vec<(Shape, Answer)> {
    //     let mut new_available: Vec<(Shape, Answer)> = Vec::new();
    //     for old_available in self.available.iter() {}
    //     new_available
    // }
    // fn get_loss(&self, shape: &Shape, answer: Answer) -> u8 {
    //     (self.available.len() - self.erase_unavailable(shape, &answer).len()) as u8
    // }
    pub fn judge(&mut self, shape: &Shape) -> Answer {
        let mut answer = Answer::default();
        if !self.hardcore {
            if shape.figure == self.shape.figure {
                answer.add_correct();
            }
            if shape.color == self.shape.color {
                answer.add_correct();
            }
            if shape.dot_position == self.shape.dot_position {
                answer.add_correct();
            }
            if shape.dot_color == self.shape.dot_color {
                answer.add_correct();
            }
        } else {
            // let mut tmp_answer = Answer::default();
            // let mut best_answer = Answer::default();
            // let mut best_answer_loss = u8::max_value();
            // for _ in 0..=3 {
            //     let maybe = self.get_loss(shape, tmp_answer);
            //     if maybe < best_answer_loss {
            //         best_answer = tmp_answer;
            //         best_answer_loss = maybe;
            //     }
            //     tmp_answer.add_correct();
            // }
            // answer = best_answer;
        }
        // self.available = self.erase_unavailable(&shape, &answer);
        self.judge_history.push((shape.clone(), answer));
        answer
    }
}
