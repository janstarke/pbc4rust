use super::QuadraticField;
use crate::pbc::traits::{Element};
use crate::pbc::*;
use std::rc::Rc;

impl<E, F> SuperElement<E, AtomicElement> for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type SubType = E;
}

impl<E, F> Element<ComplexElement> for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type FieldType = QuadraticField<E, F>;

    fn field(&self) -> Rc<Self::FieldType> {
        Rc::clone(&self.field)
    }

    fn double(&self) -> Self {
        Self::new(self.x.double(), self.y.double(), Rc::clone(&self.field))
    }
    fn halve(&self) -> Self {
        Self {
            x: self.x.halve(),
            y: self.y.halve(),
            field: self.field.clone(),
        }
    }

    fn square(&self) -> Self {
        self.clone()
    }
    fn is_sqrt(&self) -> bool {
        false
    }

    fn sqrt(&self) -> Option<(Self, Self)> {
        None
    }
}
