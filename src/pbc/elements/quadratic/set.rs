use crate::pbc::*;

impl<E, F> Set<i64> for Quadratic<E, F>
where
    E: Element<AtomicElement> + From<i64>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn set(mut self, x: i64) -> Self {
        let zero = self.x.field().zero_element();
        self.x = E::from(x);
        self.y = zero;
        self
    }
}


impl<'b, E, F> Set<&'b Quadratic<E, F>> for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn set(mut self, element: &Self) -> Self {
        self.x = element.x.clone();
        self.y = element.y.clone();
        self.field = element.field();
        self
    }
}
