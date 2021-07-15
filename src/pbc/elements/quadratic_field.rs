use gmp::mpz::Mpz;
use super::traits::{Field, Element};
use std::rc::Rc;
use std::marker::PhantomData;
use super::Quadratic;

#[derive(Debug, Clone, PartialEq)]
pub struct QuadraticField<E, F>
    where E: Element, F:Field<E> {
    order: Mpz,
    target_field: Rc<F>,
    phantom: PhantomData<E>
}

impl<E, F> QuadraticField<E, F>
where E: Element, F:Field<E> {
    pub fn new(order: Mpz, target_field: Rc<F>) -> QuadraticField<E, F> {
        let field = QuadraticField {
            order, target_field, phantom: PhantomData
        };
        field
    }

    pub fn zero (field: Rc<QuadraticField<E, F> >) -> Quadratic<E, F> {
        Quadratic::new(
            F::zero(field.target_field.clone()),
            F::zero(field.target_field.clone()),
            field.clone()
        )
    }

    pub fn one (field: Rc<QuadraticField<E, F> >) -> Quadratic<E, F> {
        Quadratic::new(
            F::one(field.target_field.clone()),
            F::one(field.target_field.clone()),
            field.clone()
        )
    }

    pub fn random (field: Rc<QuadraticField<E, F> >) -> Quadratic<E, F> {
        Quadratic::new(
            F::random(field.target_field.clone()),
            F::random(field.target_field.clone()),
            field.clone()
        )
    }
/*
    pub fn zero(field: Rc<QuadraticField>)   -> Zr { Zr::new(Mpz::from(0), field) }
    pub fn one(field: Rc<QuadraticField>)    -> Zr { Zr::new(Mpz::from(1), field) }
    pub fn two(field: Rc<QuadraticField>)    -> Zr { Zr::new(Mpz::from(2), field) }
    pub fn two_inverse(field: Rc<ZrField>)    -> Zr { Zr::new(field.inverse_of(&Mpz::from(2)), field) }
    pub fn random(field: Rc<ZrField>) -> Zr {
        let mut rng1 = rand::thread_rng();
        let mut rng2 = RandState::new();
        rng2.seed(Mpz::from(rng1.next_u64()));
        Zr::new(rng2.urandom(field.order()), field)
    }
    pub fn nqr(field: Rc<ZrField>) -> Zr {
        loop {
            let res = ZrField::random(field.clone());
            if ! res.is_zero() {
                if ! res.is_sqrt() {
                    return res;
                }
            }
        }
    }

    pub fn order(&self) -> &Mpz {
        &self.order
    }

    pub fn inverse_of(&self, value: &Mpz) -> Mpz {
        value.invert(self.order()).expect("unable to invert")
    }

    pub fn legendre(&self, value: &Mpz) -> Mpz {
        let exp = (self.order()-Mpz::one()) / Mpz::from(2 as u32);
        value.powm(&exp, self.order())
    }
*/
}