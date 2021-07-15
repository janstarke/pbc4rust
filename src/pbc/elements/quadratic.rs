use std::ops::{Add, Mul, Sub, Div};
use num_traits::{One, Zero};
use std::rc::Rc;
use super::QuadraticField;
use super::traits::{Field, Element};

#[derive(Debug, Clone, PartialEq)]
pub struct Quadratic<E: Element, F: Field<E>> {
    x: E,
    y: E,
    field: Rc<QuadraticField<E, F>>
}

impl<E, F>  Quadratic<E,F>
    where   E: Element,
            F: Field<E> {
    pub fn new(x: E, y: E, field: Rc<QuadraticField<E, F>>) -> Quadratic<E, F> {
        Quadratic {
            x,
            y,
            field
        }
    }
}
/*
impl<E, F> Element for Quadratic<E, F> {

}
*/

impl<E, F> Add for Quadratic<E, F> 
where   E: Element,
        F: Field<E> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.field, rhs.field);
        Self::new(self.x + rhs.x, self.y + rhs.y, self.field )
    }
}

impl<'a, 'b, E, F> Add<&'b Quadratic<E, F>> for &'a Quadratic<E, F> 
where   E: Element,
        for <'c> &'c E: Add<Output=E>,
        F: Field<E>,
        'a : 'b {
    type Output = Quadratic<E, F>;

    fn add(self, rhs: &'b Quadratic<E, F>) -> Self::Output {
        assert_eq!(self.field, rhs.field);
        let x = &self.x + &rhs.x;
        let y = &self.y + &rhs.y;
        Quadratic::new(x, y, self.field.clone())
    }
}

impl<E, F> Sub for Quadratic<E, F> 
where   E: Element + Sub<Output = E>,
        F: Field<E> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.x - rhs.x, self.y - rhs.y, self.field )}
}

#[cfg(test)]
mod tests {
    use gmp::mpz::Mpz;
    use super::*;
    use crate::Zr;
    use crate::ZrField;
    use std::ops::*;
    use crate::pbc::testlib::algebra::*;
    use crate::test_one;
    use crate::test_zero;
    use crate::test_associativity;
    use crate::test_commutativity;
    use crate::test_double_and_halve;
    use crate::test_distributivity;
    use crate::test_square_and_sqrt;

    type TestedField = QuadraticField<Zr, ZrField>;
    type TestedElement = Quadratic<Zr, ZrField>;

    fn field() -> Rc<TestedField> {
        let zr_field = Rc::new(ZrField::new(Mpz::from(ORDER)));
        let q_order = ZrField::nqr(zr_field.clone());
        Rc::new(QuadraticField::new(q_order.value().clone(), zr_field))
    }
    
    //test_one!(TestedElement, TestedField, field());
    test_zero!(TestedElement, TestedField, field());
    
    //test_double_and_halve!(Zr, ZrField, field());
    //test_square_and_sqrt!(Zr, ZrField, field());
    test_commutativity!(TestedElement, add, TestedField, field());
    //test_commutativity!(Zr, mul, ZrField, field());
    test_associativity!(TestedElement, add, TestedField, field());
    //test_associativity!(Zr, mul, ZrField, field());
    //test_distributivity!(Zr, add, mul, ZrField, field());
    
}