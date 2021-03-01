use gmp::mpz::Mpz;
use super::Zr;
use num_traits::{One, Zero};
use std::rc::Rc;
use gmp::rand::RandState;
use rand::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ZrField {
    order: Mpz,
}

impl ZrField {
    pub fn new(order: Mpz) -> ZrField {
        let field = ZrField {
            order: order
        };
        field
    }

    pub fn two_inverse(&self)         -> Mpz { self.inverse_of(&Mpz::from(2)) }

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

    pub fn zero(field: Rc<ZrField>)   -> Zr { Zr::new(Mpz::from(0), field) }
    pub fn one(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(1), field) }
    pub fn two(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(2), field) }
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
}