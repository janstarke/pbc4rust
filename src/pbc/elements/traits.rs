use num_traits::{One, Zero};
use std::rc::Rc;
use gmp::mpz::Mpz;
use std::ops::*;


pub trait Element: One + Zero + Clone + PartialEq + Add + Mul + Div + Sub + Neg {
    type FieldType;
    fn field(&self) -> Option<Rc<Self::FieldType>>;

    //fn inverse(&self) -> Self;
    fn square(&self) -> Self;
    fn double(&self) -> Self;
    fn halve(&self) -> Self;
    fn is_sqrt(&self) -> bool;
    fn sqrt(&self) -> Option<(Self, Self)>;
    //fn mulZn(&self, n: &Zr) -> Self;
    //fn pow(&self, exp: &Mpz) -> Self;
    //fn powZn(&self, exp: &Zr) -> Self;
}

pub trait Field<E:Element> {
    fn zero(field: Rc<Self>) -> E;
    fn one(field: Rc<Self>) -> E;
    fn random(field: Rc<Self>) -> E;
}

pub trait FiniteField<E:Element> : Field<E> {
    fn order(&self) -> &Mpz;
}

pub trait FieldOver<E:Element> : Field<E> {
    fn target_field<F>(&self) -> Rc<F>;
}