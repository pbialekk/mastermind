use crate::app::{Answer, Shape};

#[derive(Clone, Debug)]
pub struct Judge {
    shape: Shape,
    judge_history: Vec<(Shape, Answer)>,
}

impl Judge {
    pub fn new() -> Self {
        Self {
            shape: Shape::random(),
            judge_history: Vec::new(),
        }
    }
    pub fn judge(&mut self, shape: &Shape) -> Answer {
        let mut answer = Answer::default();
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
        self.judge_history.push((shape.clone(), answer.clone()));
        answer
    }
}
