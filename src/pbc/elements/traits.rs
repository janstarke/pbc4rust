use gmp::mpz::Mpz;
use std::fmt::Debug;
use std::ops::*;
use std::rc::Rc;

pub trait HasZero<E: CanBeZero> {
    fn zero_element(self: Rc<Self>) -> E;
}

pub trait HasOne<E: CanBeOne> {
    fn one_element(self: Rc<Self>) -> E;
}

pub trait CanBeZero {
    fn is_zero(&self) -> bool;
}

pub trait CanBeOne {
    fn is_one(&self) -> bool;
}

pub trait ElementLevel {}
pub enum AtomicElement {}
pub enum ComplexElement {}
impl ElementLevel for AtomicElement {}
impl ElementLevel for ComplexElement {}

pub trait Element<T: ElementLevel>:
    Debug
    + Clone
    + PartialEq
    + CanBeZero
    + CanBeOne
    + Add<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + Mul<Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + Sub<Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + Neg<Output = Self>
    + for<'b> Set<&'b Self>
{
    type FieldType: Field<Self, T>;
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

pub trait SuperElement<E, T> : Element<ComplexElement> 
where E: Element<T>, T: ElementLevel {
    type SubType: Element<T>;
}

pub trait Set<V>
{
    fn set(self, value: V) -> Self;
}

pub trait Field<E: Element<T>, T>: Debug + Clone + HasZero<E> + HasOne<E> + PartialEq
where
    T: ElementLevel,
{
    //type ElementType: Element<T>;
    fn random_element(self: Rc<Self>) -> E;
}

pub trait FiniteField<E: Element<T>, T>: Field<E, T>
where
    T: ElementLevel,
{
    fn order(&self) -> &Mpz;
}

pub trait FieldOver<E, F, E2, G, T>: Field<E, ComplexElement>
where
    G: Field<E2, T>,
    E: SuperElement<E2, T>,
    E2: Element<T>,
    T: ElementLevel,
{
    fn target_field(&self) -> Rc<G>;
}

pub trait HasNqr<E: Element<T>, T>: Field<E, T>
where
    T: ElementLevel,
{
    fn nqr(field: Rc<Self>) -> E;
}
