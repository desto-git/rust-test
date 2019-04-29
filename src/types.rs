pub enum Direction { Up, Down, Left, Right }

#[derive(Clone, Copy)]
pub struct Coordinate { pub x: i8, pub y: i8 }

#[derive(Clone, Copy)]
pub struct Dimension<T> { pub w: T, pub h: T }