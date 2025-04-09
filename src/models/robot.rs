use crate::models::position::Position;


#[derive(Clone, Debug)]
pub struct Robot {
    pub id: usize,
    pub position: Position,
    pub has_resource: bool,
}

pub struct GameState {
    pub map: Vec<Vec<char>>,
    pub robots: Vec<Robot>,
    pub finished_robots: usize,
}