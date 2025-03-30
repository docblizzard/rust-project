use crate::models::position::Position;


#[derive(Clone, Copy, Debug)]
pub struct Robot {
    id: usize,
    position: Position,
    path: Vec<Position>,
    state: RobotState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RobotState {
    Searching,
    Returning,
    Done,
}