pub(crate) struct Mat3 {
    values: [fn(f32) -> f32; 9],
}

impl Mat3 {
    pub fn new(values: [fn(f32) -> f32; 9]) -> Self {
        Mat3 { values }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<fn(f32) -> f32> {
        if row < 3 && col < 3 {
            Some(self.values[row * 3 + col])
        } else {
            None
        }
    }

    pub fn get_and_eval(&self, row: usize, col: usize, at: f32) -> Option<f32> {
        let func = self.get(row, col)?;
        Some(func(at))
    }
}
