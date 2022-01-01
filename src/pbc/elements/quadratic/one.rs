use crate::pbc::*;

impl<E, F> CanBeOne for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_zero()
    }
}