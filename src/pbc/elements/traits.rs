use std::rc::Rc;
use gmp::mpz::Mpz;
use std::ops::*;
use std::fmt::Debug;

pub trait HasZero<E:CanBeZero> {
    fn zero_element(self: Rc<Self>) -> E;
}

pub trait HasOne<E:CanBeOne> {
    fn one_element(self: Rc<Self>) -> E;
}

pub trait CanBeZero {
    fn is_zero(&self) -> bool;
}

pub trait CanBeOne {
    fn is_one(&self) -> bool;
}

pub trait ElementType {}
pub enum AtomicElement {}
pub enum ComplexElement {}
impl ElementType for AtomicElement {}
impl ElementType for ComplexElement {}

pub trait Element<E: ElementType>: 
        Debug + 
        Clone + 
        PartialEq +
        CanBeZero +
        CanBeOne +
        Add<Output=Self> + for<'a> Add<&'a Self, Output=Self> +
        Mul<Output=Self> + for<'a> Mul<&'a Self, Output=Self> + 
        Sub<Output=Self> + for<'a> Sub<&'a Self, Output=Self> + 
        Neg<Output=Self>
        {
    type FieldType;
    fn field(&self) -> Rc<Self::FieldType>;

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

pub trait Field<E:Element<T>, T>: Debug + Clone + HasZero<E> + HasOne<E> + PartialEq where T: ElementType {
    fn random_element(self: Rc<Self>) -> E;
}

pub trait FiniteField<E:Element<T>, T> : Field<E, T> where T: ElementType  {
    fn order(&self) -> &Mpz;
}

pub trait FieldOver<E:Element<T>, T> : Field<E, T> where T: ElementType  {
    fn target_field<F>(&self) -> Rc<F>;
}