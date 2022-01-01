use crate::pbc::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Quadratic<E: Element<AtomicElement>, F: HasZero<E> + HasNqr<E, AtomicElement>> {
    pub x: E,
    pub y: E,
    pub field: Rc<QuadraticField<E, F>>,
}

impl<E, F> Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    pub fn new(x: E, y: E, field: Rc<QuadraticField<E, F>>) -> Quadratic<E, F> {
        Self { x, y, field }
    }
}