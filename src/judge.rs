use crate::app::{Answer, Shape};
use crate::solver;

#[derive(Clone, Debug)]
pub struct Judge {
    shape: Shape,
    judge_history: Vec<(Shape, Answer)>,
    hardcore: bool,
}

impl Judge {
    fn new() -> Self {
        Self {
            shape: Shape::random(),
            judge_history: Vec::new(),
            hardcore: std::env::args().any(|arg| arg == "--hardcore"),
        }
    }
    pub fn judge_normal(&mut self, shape: &Shape) -> Answer {
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
        self.judge_history.push((*shape, answer));
        answer
    }
    pub fn judge_hardcore(&mut self, shape: &Shape) -> Answer {
        if self.judge_history.is_empty() {
            let mut ans = Answer::default();
            ans.add_correct();
            self.judge_history.push((*shape, ans));
            return ans;
        }
        let mut answer = Answer::default();
        let mut best_answer = answer;
        let mut best_answer_dep = u8::min_value();
        for _ans in 0..=4 {
            self.judge_history.push((*shape, answer));
            if let Some(dep) = solver::get_min_dep(&self.judge_history) {
                if dep > best_answer_dep {
                    best_answer = answer;
                    best_answer_dep = dep;
                }
            }
            self.judge_history.pop();
            answer.add_correct();
        }
        self.judge_history.push((*shape, best_answer));
        best_answer
    }
    pub fn judge(&mut self, shape: &Shape) -> Answer {
        if self.hardcore {
            self.judge_hardcore(shape)
        } else {
            self.judge_normal(shape)
        }
    }
}

impl Default for Judge {
    fn default() -> Self {
        Self::new()
    }
}
