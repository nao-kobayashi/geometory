use crate::line::Line;

#[derive(Debug, PartialEq, Clone)]
pub struct SweepLine {
    sweep_line: Line,
    below_line: Line,
}

impl SweepLine {
    pub fn new() -> Self {
        Self {
            sweep_line: Line::from_points(0_f64, 0_f64, 1_f64, 0_f64),
            below_line: Line::from_points(0_f64, 0.1, 1_f64, 0.1),
        }
    }

    pub fn set_y(&mut self, y: f64) {
        self.sweep_line = Line::from_points(0_f64, y, 1_f64, y);
        self.below_line = Line::from_points(0_f64, y + 0.1, 1_f64, y + 0.1);
    }

    pub fn get_sweep_line(&self) -> &Line {
        &self.sweep_line
    }

    pub fn get_below_line(&self) -> & Line {
        &self.below_line
    }
}
