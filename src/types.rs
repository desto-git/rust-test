pub enum Direction { Up, Down, Left, Right }

#[derive(Clone, Copy)]
pub struct Coordinate<T> { pub x: T, pub y: T }

#[derive(Clone, Copy)]
pub struct Dimension<T> { pub w: T, pub h: T }