use super::Position;

pub trait Ship{
    fn position(&self)->Position;
}
