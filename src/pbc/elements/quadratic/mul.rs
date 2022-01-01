use crate::pbc::*;
use std::ops::Mul;

macro_rules! implement_mul {
    ($lhs:tt, $rhs:tt) => {
        {
            let mut e0 = $lhs.x.clone() + &$lhs.y;
            let mut e1 = $rhs.x.clone() + &$rhs.y;
            let e2 = e0.clone() * &e1;

            e0 = e0.set(&$lhs.x);
            e0 = e0 * &$rhs.x;

            e1 = e1.set(&$lhs.y);
            e1 = e1 * &$rhs.y;
            
            let nqr = F::nqr($lhs.field().target_field());

            let x = (e1.clone() * nqr) + &e0;
            let y = (e2 - e0) - e1;

            Quadratic {x, y, field: $lhs.field()}
        }
    };
}

impl<'a, 'b, E, F> Mul<&'b Quadratic<E, F>> for &'a Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type Output = Quadratic<E, F>;
    fn mul(self, rhs: &'b Quadratic<E, F>) -> Self::Output {
        implement_mul!(self, rhs)
    }
}

impl<'a, E, F> Mul<Quadratic<E, F>> for &'a Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type Output = Quadratic<E, F>;
    fn mul(self, rhs: Quadratic<E, F>) -> Self::Output {
        implement_mul!(self, rhs)
    }
}

impl<'b, E, F> Mul<&'b Quadratic<E, F>> for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type Output = Quadratic<E, F>;
    fn mul(self, rhs: &'b Quadratic<E, F>) -> Self::Output {
        implement_mul!(self, rhs)
    }
}

impl<E, F> Mul<Quadratic<E, F>> for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    type Output = Quadratic<E, F>;
    fn mul(self, rhs: Quadratic<E, F>) -> Self::Output {
        implement_mul!(self, rhs)
    }
}
