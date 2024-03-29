use crate::pbc::*;
use std::ops::Mul;

macro_rules! implement_mul {
    ($lhs:tt, $rhs:tt) => {
        {
            let nqr = F::nqr($lhs.field().target_field());
            /* naive */
            let e0 = $lhs.x.clone() * &$rhs.x;
            let e1 = ($lhs.y.clone() * &$rhs.y);
            let e1 = e1 * nqr;
            let e0 = e0 + e1;

            let e1 = $lhs.x.clone() * &$rhs.y;
            let e2 = $lhs.y.clone() * &$rhs.x;
            let e1 = e1 + e2;

            let x = e0;
            let y = e1;
            /* Implementation of the Karatsuba alorithm */
            /*
            let e0 = $lhs.x.clone() + &$lhs.y;
            let e1 = $rhs.x.clone() + &$rhs.y;
            let e2 = e0.clone() * &e1;

            let e0 = $lhs.x.clone() * &$rhs.x;
            let e1 = $lhs.y.clone() * &$rhs.y;
            
            let nqr = F::nqr($lhs.field().target_field());

            let x = (e1.clone() * nqr) + &e0;
            let y = (e2 - e0) - e1;
            */

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
        let nqr = F::nqr(self.field().target_field());
        /* naive */
        let e0 = self.x.clone() * &rhs.x;
        let e1 = (self.y.clone() * &rhs.y);
        let e1 = e1 * &nqr;
        let e0 = e0 + e1;

        let e1 = self.x.clone() * &rhs.y;
        let e2 = self.y.clone() * &rhs.x;
        let e1 = e1 + e2;

        let x = e0;
        let y = e1;
        /* Implementation of the Karatsuba alorithm */
        /*
        let e0 = $lhs.x.clone() + &$lhs.y;
        let e1 = $rhs.x.clone() + &$rhs.y;
        let e2 = e0.clone() * &e1;

        let e0 = $lhs.x.clone() * &$rhs.x;
        let e1 = $lhs.y.clone() * &$rhs.y;
        
        let nqr = F::nqr($lhs.field().target_field());

        let x = (e1.clone() * nqr) + &e0;
        let y = (e2 - e0) - e1;
        */

        Quadratic {x, y, field: self.field()}
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
