use gmp::mpz::Mpz;
use super::Z;
use rand::*;
use crate::pbc::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct ZField();

impl ZField {
    pub fn new() -> ZField {
        ZField {}
    }
}

impl HasOne<Z> for ZField {
    fn one_element(self: Rc<Self>) -> Z {
        Z::from(1)
    }
}

impl HasZero<Z> for ZField {
    fn zero_element(self: Rc<Self>) -> Z {
        Z::from(0)
    }
}

impl Field<Z, AtomicElement> for ZField {
    fn random_element(self: Rc<Self>) -> Z {
        let mut rng1 = rand::thread_rng();
        Z::new(Mpz::from(rng1.next_u64()))
    }
}