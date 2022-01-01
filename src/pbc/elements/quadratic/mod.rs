use super::traits::{Element, FiniteField};
use super::QuadraticField;
use crate::pbc::*;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

pub mod quadratic;
pub mod one;
pub mod zero;
pub mod neg;
pub mod set;
pub mod element;
pub mod mul;

pub use quadratic::Quadratic;
pub use one::*;
pub use zero::*;
pub use neg::*;
pub use set::*;
pub use element::*;
pub use mul::*;

#[allow(unused_macros)]
macro_rules! add_operator {
    ($op:tt, $trait:tt, $method: tt ) => {
        impl<E, F> $trait<Self> for Quadratic<E, F>
        where E: Element<AtomicElement>,
        F: HasZero<E> + HasNqr<E, AtomicElement>, {
            type Output=Quadratic<E, F>;
            fn $method(self, rhs: Self) -> Self::Output {
                Quadratic::new(self.x $op &rhs.x, self.y $op &rhs.y, self.field.clone())
            }
        }

        impl<'b, E, F> $trait<&'b Self> for Quadratic<E, F>
        where E: Element<AtomicElement>, 
        F: HasZero<E> + HasNqr<E, AtomicElement>, {
            type Output=Quadratic<E, F>;
            fn $method(self, rhs: &Self) -> Self::Output {
                Quadratic::new(self.x $op &rhs.x, self.y $op &rhs.y, self.field.clone())
            }
        }

        impl<'a, 'b, E, F> $trait<&'b Quadratic<E, F>> for &'a Quadratic<E, F>
        where E: Element<AtomicElement>, 
        F: HasZero<E> + HasNqr<E, AtomicElement>, {
            type Output=Quadratic<E, F>;
            fn $method(self, rhs: &'b Quadratic<E, F>) -> Self::Output {
                Quadratic::new(self.x.clone() $op &rhs.x, self.y.clone() $op &rhs.y, self.field.clone())
            }
        }
    };
}

add_operator!(+, Add, add);
add_operator!(-, Sub, sub);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::pbc::testlib::algebra::*;
    use crate::test_associativity;
    use crate::test_commutativity;
    use crate::test_distributivity;
    use crate::test_double_and_halve;
    use crate::test_one;
    //use crate::test_square_and_sqrt;
    use crate::pbc::Zr;
    use crate::pbc::ZrField;
    use crate::test_zero;
    use gmp::mpz::Mpz;

    fn field() -> Rc<QuadraticField<Zr, ZrField>> {
        let zr_field = Rc::new(ZrField::new(Mpz::from(ORDER)));
        //let q_order = ZrField::nqr(zr_field.clone());
        Rc::new(QuadraticField::new(zr_field))
    }
    test_one!(Quadratic<Zr, ZrField>, QuadraticField<Zr, ZrField>, field());
    test_zero!(Quadratic<Zr, ZrField>, QuadraticField<Zr, ZrField>, field());
    //test_double_and_halve!(Quadratic<Zr, ZrField>, QuadraticField<Zr, ZrField>, field());
    //test_square_and_sqrt!(Quadratic<Zr, ZrField>, QuadraticField<Zr, ZrField>, field());
    test_commutativity!(Quadratic<Zr, ZrField>, add, QuadraticField<Zr, ZrField>, field());
    test_commutativity!(Quadratic<Zr, ZrField>, mul, QuadraticField<Zr, ZrField>, field());
    test_associativity!(Quadratic<Zr, ZrField>, add, QuadraticField<Zr, ZrField>, field());
    test_associativity!(Quadratic<Zr, ZrField>, mul, QuadraticField<Zr, ZrField>, field());
    test_distributivity!(Quadratic<Zr, ZrField>, add, mul, QuadraticField<Zr, ZrField>, field());
}
