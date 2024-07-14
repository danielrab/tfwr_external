#[derive(Debug, Default, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn wrapped_manhattan_distance(&self, other: &Position, size_x: usize, size_y: usize) -> usize {
        let dx = (self.x as isize - other.x as isize).unsigned_abs();
        let dy = (self.y as isize - other.y as isize).unsigned_abs();
        let dx_wrapped = dx.min(size_x - dx);
        let dy_wrapped = dy.min(size_y - dy);
        dx_wrapped + dy_wrapped
    }
}