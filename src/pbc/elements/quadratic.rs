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

macro_rules! _parse_binary_op {
    (+, $($t:tt)+) => (quadratic_op_internal!(Add, add, $($t)+););
    (-, $($t:tt)+) => (quadratic_op_internal!(Sub, sub, $($t)+););
    (*, $($t:tt)+) => (quadratic_op_internal!(Mul, mul, $($t)+););
    (/, $($t:tt)+) => (quadratic_op_internal!(Div, div, $($t)+););
}
macro_rules! quadratic_op_internal {
    ($ops_trait:ident, $ops_fn:ident, &$lhs_i:ident, &$rhs_i:ident, $body:block) => (
        quadratic_op_borrowed_borrowed!($ops_trait, $ops_fn, $lhs_i, $rhs_i, $body);
    );
    ($ops_trait:ident, $ops_fn:ident, &$lhs_i:ident, $rhs_i:ident, $body:block) => (
        quadratic_op_borrowed_owned!($ops_trait, $ops_fn, $lhs_i, $rhs_i, $body);
    );
    ($ops_trait:ident, $ops_fn:ident, $lhs_i:ident, &$rhs_i:ident, $body:block) => (
        quadratic_op_owned_borrowed!($ops_trait, $ops_fn, $lhs_i, $rhs_i, $body);
    );
    ($ops_trait:ident, $ops_fn:ident, $lhs_i:ident, $rhs_i:ident, $body:block) => (
        quadratic_op_owned_owned!($ops_trait, $ops_fn, $lhs_i, $rhs_i, $body);
    );
}
macro_rules! quadratic_op_owned_owned {
    ($ops_trait:ident, $ops_fn:ident, $lhs_i:ident, $rhs_i:ident, $body: block) => {
        impl<E, F> $ops_trait for Quadratic<E, F>
        where   E: Element,
                for <'c> &'c E: $ops_trait<Output=E>,
                F: Field<E> {
            type Output = Quadratic<E, F>;

            fn $ops_fn (self, $rhs_i: Quadratic<E, F>) -> Self::Output {
                let $lhs_i = self;
                assert_eq!($lhs_i.field, $rhs_i.field);
                $body
            }
        }
    };
}
macro_rules! quadratic_op_owned_borrowed {
    ($ops_trait:ident, $ops_fn:ident, $lhs_i:ident, $rhs_i:ident, $body: block) => {
        impl<'b, E, F> $ops_trait<&'b Quadratic<E, F>> for Quadratic<E, F>
        where   E: Element,
                for <'c> &'c E: $ops_trait<Output=E>,
                F: Field<E> {
            type Output = Quadratic<E, F>;

            fn $ops_fn (self, $rhs_i: &'b Quadratic<E, F>) -> Self::Output {
                let $lhs_i = self;
                assert_eq!($lhs_i.field, $rhs_i.field);
                $body
            }   
        }
    };
}
macro_rules! quadratic_op_borrowed_owned {
    ($ops_trait:ident, $ops_fn:ident, $lhs_i:ident, $rhs_i:ident, $body: block) => {
        impl<'a, E, F> $ops_trait<Quadratic<E, F>> for &'a Quadratic<E, F>
        where   E: Element,
                for <'c> &'c E: $ops_trait<Output=E>,
                F: Field<E> {
            type Output = Quadratic<E, F>;

            fn $ops_fn (self, $rhs_i: Quadratic<E, F>) -> Self::Output {
                let $lhs_i = self;
                assert_eq!($lhs_i.field, $rhs_i.field);
                $body
            }   
        }
    };
}
macro_rules! quadratic_op_borrowed_borrowed {
    ($ops_trait:ident, $ops_fn:ident, $lhs_i:ident, $rhs_i:ident, $body: block) => {
        impl<'a, 'b, E, F> $ops_trait<&'b Quadratic<E, F>> for &'a Quadratic<E, F>
        where   E: Element,
                for <'c> &'c E: $ops_trait<Output=E>,
                F: Field<E> {
            type Output = Quadratic<E, F>;

            fn $ops_fn (self, $rhs_i: &'b Quadratic<E, F>) -> Self::Output {
                let $lhs_i = self;
                assert_eq!($lhs_i.field, $rhs_i.field);
                $body
            }   
        }
    };
}

macro_rules! quadratic_op {
    ($op:tt |$lhs_i:ident , $rhs_i:ident| $body:block) => {
        _parse_binary_op!($op,  lhs,  rhs, {|$lhs_i :  Quadratic<E, F>, $rhs_i :  Quadratic<E, F>| -> Quadratic<E, F> {$body}( lhs,  rhs)});
        _parse_binary_op!($op,  lhs, &rhs, {|$lhs_i :  Quadratic<E, F>, $rhs_i : &Quadratic<E, F>| -> Quadratic<E, F> {$body}( lhs, &rhs)});
        _parse_binary_op!($op, &lhs,  rhs, {|$lhs_i : &Quadratic<E, F>, $rhs_i :  Quadratic<E, F>| -> Quadratic<E, F> {$body}(&lhs,  rhs)});
        _parse_binary_op!($op, &lhs, &rhs, {|$lhs_i : &Quadratic<E, F>, $rhs_i : &Quadratic<E, F>| -> Quadratic<E, F> {$body}(&lhs, &rhs)});
    };
}

quadratic_op!(+ |lhs, rhs| {Quadratic::new(&lhs.x + &rhs.x, &lhs.y + &rhs.y, lhs.field.clone() )});
quadratic_op!(- |lhs, rhs| {Quadratic::new(&lhs.x - &rhs.x, &lhs.y - &rhs.y, lhs.field.clone() )});
quadratic_op!(* |lhs, rhs| {Quadratic::new(&lhs.x * &rhs.x, &lhs.y * &rhs.y, lhs.field.clone() )});


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


    fn field() -> Rc<QuadraticField<Zr, ZrField>> {
        let zr_field = Rc::new(ZrField::new(Mpz::from(ORDER)));
        let q_order = ZrField::nqr(zr_field.clone());
        Rc::new(QuadraticField::new(q_order.value().clone(), zr_field))
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