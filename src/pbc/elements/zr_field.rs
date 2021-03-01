use gmp::mpz::Mpz;
use super::Zr;
use num_traits::Zero;
use std::rc::Rc;
use gmp::rand::RandState;
use rand::*;
use super::traits::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ZrField {
    order: Mpz,
}

impl Field<Zr> for ZrField {
    fn zero(field: Rc<ZrField>)   -> Zr { Zr::new(Mpz::from(0), field) }
    fn one(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(1), field) }
    fn random(field: Rc<ZrField>) -> Zr {
        let mut rng1 = rand::thread_rng();
        let mut rng2 = RandState::new();
        rng2.seed(Mpz::from(rng1.next_u64()));
        Zr::new(rng2.urandom(field.order()), field)
    }
}

impl FiniteField<Zr> for ZrField {
    fn order(&self) -> &Mpz { &self.order }
}

impl ZrField {
    pub fn new(order: Mpz) -> ZrField {
        let field = ZrField {
            order: order
        };
        field
    }

    pub fn two_inverse(&self)         -> Mpz { self.inverse_of(&Mpz::from(2)) }

    pub fn inverse_of(&self, value: &Mpz) -> Mpz {
        value.invert(self.order()).expect("unable to invert")
    }

    pub fn legendre(&self, value: &Mpz) -> Mpz {
        let exp = (self.order()-Mpz::one()) / Mpz::from(2 as u32);
        value.powm(&exp, self.order())
    }

    pub fn two(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(2), field) }
    
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