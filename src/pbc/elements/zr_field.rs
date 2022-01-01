use gmp::mpz::Mpz;
use super::Zr;
use std::rc::Rc;
use gmp::rand::RandState;
use rand::*;
use super::traits::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ZrField {
    order: Mpz,
}

impl Field<Zr, AtomicElement> for ZrField {
    
    fn random_element(self: Rc<Self>) -> Zr {
        let mut rng1 = rand::thread_rng();
        let mut rng2 = RandState::new();
        rng2.seed(Mpz::from(rng1.next_u64()));
        Zr::new(rng2.urandom(self.order()), Rc::clone(&self))
    }
}

impl HasOne<Zr> for ZrField {
    fn one_element(self: Rc<Self>) -> Zr {
        Zr::new(Mpz::from(1), Rc::clone(&self))
    }
}

impl HasZero<Zr> for ZrField {
    fn zero_element(self: Rc<Self>) -> Zr {
        Zr::new(Mpz::from(0), Rc::clone(&self))
    }
}

impl FiniteField<Zr, AtomicElement> for ZrField {
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

    pub fn two(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(2), Rc::clone(&field)) }
}

impl HasNqr<Zr, AtomicElement> for ZrField {
    fn nqr(field: Rc<ZrField>) -> Zr {
        loop {
            let res = ZrField::random_element(Rc::clone(&field));
            if ! res.is_zero() {
                if ! res.is_sqrt() {
                    return res;
                }
            }
        }
    }
}