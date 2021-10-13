#[derive(Debug, Copy, Clone)]
pub enum Direction{
    UP,
    RIGHT,
    DOWN,
    LEFT
}
impl Direction {
    pub fn get_x(&self) -> i64{
        match self{
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 0,
            Self::LEFT => -1,
        }
    }
    pub fn get_y(&self) -> i64{
        match self{
            Self::UP => 1,
            Self::RIGHT => 0,
            Self::DOWN => -1,
            Self::LEFT => 0,
        }
    }
}