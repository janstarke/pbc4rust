use gmp::mpz::{Mpz,ProbabPrimeResult};
use super::Zr;
use std::rc::Rc;
use gmp::rand::RandState;
use rand::*;
use super::traits::*;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct ZrField {
    order: Mpz,
    nqr: Mpz
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
        assert!(order.probab_prime(10) == ProbabPrimeResult::Prime);

        let nqr = {
            let tmp_field = Rc::new(ZrField {
                order: order.clone(),
                nqr: Mpz::zero()
            });

            let nqr;
            loop {
                let res = ZrField::random_element(Rc::clone(&tmp_field));
                if ! (res.is_zero() || res.is_one()) {
                    if ! res.is_sqrt() {
                        nqr = res;
                        break;
                    }
                }
            }
            nqr.value().clone()
        };

        let field = ZrField {
            order: order,
            nqr: nqr
        };
        
        field
    }

    pub fn two_inverse(&self) -> Mpz { self.inverse_of(&Mpz::from(2)) }

    pub fn inverse_of(&self, value: &Mpz) -> Mpz {
        value.invert(self.order()).expect("unable to invert")
    }

    pub fn legendre(&self, value: &Mpz) -> Mpz {
        let exp = (self.order()-Mpz::one()) / Mpz::from(2 as u32);
        value.powm(&exp, self.order())
    }

    pub fn two(field: Rc<ZrField>) -> Zr { Zr::new(Mpz::from(2), Rc::clone(&field)) }
}

impl HasNqr<Zr, AtomicElement> for ZrField {
    fn nqr(field: Rc<Self>) -> Zr {
        Zr::new(field.nqr.clone(), Rc::clone(&field))
    }
}

impl PartialEq for ZrField {
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order && self.nqr == other.nqr
    }
}