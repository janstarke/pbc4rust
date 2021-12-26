use super::traits::{FiniteField, Element};
use super::QuadraticField;
use std::ops::{Add, Mul, Neg, Sub};
use std::rc::Rc;
use crate::pbc::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Quadratic<E: Element<AtomicElement>, F: FiniteField<E, AtomicElement>> {
    x: E,
    y: E,
    field: Rc<QuadraticField<E, F>>
}

impl<E, F> Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: FiniteField<E,AtomicElement>,
{
    pub fn new(x: E, y: E, field: Rc<QuadraticField<E, F>>) -> Quadratic<E, F> {
        Self {
            x, y, field 
        }
    }
}

impl<E, F> CanBeOne for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: FiniteField<E,AtomicElement>,
{
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_zero()
    }
}

impl<E, F> CanBeZero for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: FiniteField<E,AtomicElement>,
{
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}
impl<E, F> Neg for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: FiniteField<E,AtomicElement>,
{
    type Output = Quadratic<E, F>;

    fn neg(self) -> Self {
        Self::new(
            self.x.neg(),
            self.y.neg(),
            Rc::clone(&self.field)
        )
    }
}

impl<E, F> Element<ComplexElement> for Quadratic<E, F>
where
    E: Element<AtomicElement>,
    F: FiniteField<E, AtomicElement>,
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

    fn square(&self) -> Self { self.clone() }
    fn is_sqrt(&self) -> bool { false }

    fn sqrt(&self) -> Option<(Self, Self)> { None }
}
/*
impl<E, F> Add<Self> for Quadratic<E, F>
where E: Element<AtomicElement>, F: FiniteField<E, AtomicElement> {
    type Output=Quadratic<E, F>;
    fn add(lhs: &Self, rhs: &Self) -> Self::Output {
        Quadratic::new(lhs.x + &rhs.x, lhs.y + &rhs.y, lhs.field.clone())
    }
}

impl<'a, E, F> Add<&'a Self> for Quadratic<E, F>
where E: Element<AtomicElement>, F: FiniteField<E, AtomicElement> {
    type Output=Quadratic<E, F>;
    fn add(lhs: &Self, rhs: &Self) -> Self::Output {
        Quadratic::new(lhs.x + &rhs.x, lhs.y + &rhs.y, lhs.field.clone())
    }
}
*/
#[allow(unused_macros)]
macro_rules! add_operator {
    ($op:tt, $trait:tt, $method: tt ) => {
        impl<E, F> $trait<Self> for Quadratic<E, F>
        where E: Element<AtomicElement>, F: FiniteField<E, AtomicElement> {
            type Output=Quadratic<E, F>;
            fn $method(self, rhs: Self) -> Self::Output {
                Quadratic::new(self.x $op &rhs.x, self.y $op &rhs.y, self.field.clone())
            }
        }

        impl<'b, E, F> $trait<&'b Self> for Quadratic<E, F>
        where E: Element<AtomicElement>, F: FiniteField<E, AtomicElement> {
            type Output=Quadratic<E, F>;
            fn $method(self, rhs: &Self) -> Self::Output {
                Quadratic::new(self.x $op &rhs.x, self.y $op &rhs.y, self.field.clone())
            }
        }

        impl<'a, 'b, E, F> $trait<&'b Quadratic<E, F>> for &'a Quadratic<E, F>
        where E: Element<AtomicElement>, F: FiniteField<E, AtomicElement> {
            type Output=Quadratic<E, F>;
            fn $method(self, rhs: &'b Quadratic<E, F>) -> Self::Output {
                Quadratic::new(self.x.clone() $op &rhs.x, self.y.clone() $op &rhs.y, self.field.clone())
            }
        }
    };
}

add_operator!(+, Add, add);
add_operator!(-, Sub, sub);
add_operator!(*, Mul, mul);
//add_operators!(+-*/);

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
