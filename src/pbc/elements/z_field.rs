use std::rc::Rc;
use gmp::mpz::Mpz;
use super::Z;
use rand::*;

pub struct ZField();

impl ZField {
    pub fn new() -> ZField {
        ZField {}
    }

    pub fn zero(_: Rc<Self>)   -> Z { Z::new(Mpz::from(0)) }
    pub fn one(_: Rc<Self>)    -> Z { Z::new(Mpz::from(1)) }
    pub fn random(_: Rc<Self>) -> Z {
        let mut rng1 = rand::thread_rng();
        Z::new(Mpz::from(rng1.next_u64()))
    }
}