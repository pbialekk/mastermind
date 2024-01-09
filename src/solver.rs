use crate::app::{Answer, Color, DotPosition, Figure, Shape};

fn get_available_answers(ans_hist: &Vec<(Shape, Answer)>) -> Vec<Shape> {
    fn get_intersection(a: &Shape, b: &Shape) -> u8 {
        let mut ans = 0;
        if a.figure == b.figure {
            ans += 1;
        }
        if a.color == b.color {
            ans += 1;
        }
        if a.dot_position == b.dot_position {
            ans += 1;
        }
        if a.dot_color == b.dot_color {
            ans += 1;
        }
        ans
    }
    fn get_all_shapes() -> Vec<Shape> {
        let mut ans = Vec::new();
        const FIGURES: [Figure; 3] = [Figure::Circle, Figure::Rectangle, Figure::Triangle];
        const COLORS: [Color; 3] = [Color::Blue, Color::Green, Color::Red];
        const DOT_POSITIONS: [DotPosition; 3] =
            [DotPosition::Left, DotPosition::Middle, DotPosition::Right];
        for figure in FIGURES.iter() {
            for color in COLORS.iter() {
                for dot_position in DOT_POSITIONS.iter() {
                    for dot_color in COLORS.iter() {
                        ans.push(Shape {
                            figure: *figure,
                            color: *color,
                            dot_position: *dot_position,
                            dot_color: *dot_color,
                        });
                    }
                }
            }
        }
        ans
    }
    fn is_viable(shape: &Shape, ans_hist: &Vec<(Shape, Answer)>) -> bool {
        for (s, ans) in ans_hist {
            if get_intersection(shape, s) != ans.correct {
                return false;
            }
        }
        true
    }

    let mut ans = Vec::new();
    let binding = get_all_shapes();
    for shape in binding.iter() {
        if is_viable(shape, ans_hist) {
            ans.push(shape.clone());
        }
    }
    ans
}

fn dfs(mut ans_hist: &mut Vec<(Shape, Answer)>) -> u8 {
    let mut mx = u8::max_value();
    let maybe = get_available_answers(ans_hist);
    if maybe.len() == 1 {
        return 1;
    }

    for shape in maybe {
        let mut shape_mx = u8::min_value();
        for ans in 0..=4 {
            let mut answ = Answer::default();
            answ.set_correct(ans);

            ans_hist.push((shape, answ));

            if is_valid_history(&ans_hist) {
                shape_mx = shape_mx.max(1 + dfs(&mut ans_hist));
            }

            ans_hist.pop();
        }
        mx = mx.min(shape_mx);
    }
    mx
}

fn is_valid_history(ans_hist: &Vec<(Shape, Answer)>) -> bool {
    return get_available_answers(ans_hist).len() != 0;
}

pub fn get_min_dep(ans_hist: &Vec<(Shape, Answer)>) -> Option<u8> {
    if !is_valid_history(ans_hist) {
        None
    } else {
        let mut ans_hist = ans_hist.clone();
        Some(dfs(&mut ans_hist))
    }
}
