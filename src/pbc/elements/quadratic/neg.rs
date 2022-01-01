use crate::pbc::*;
use std::ops::Neg;
use std::rc::Rc;

impl<E, F> Neg for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type Output = Quadratic<E, F>;

    fn neg(self) -> Self {
        Self::new(self.x.neg(), self.y.neg(), Rc::clone(&self.field))
    }
}