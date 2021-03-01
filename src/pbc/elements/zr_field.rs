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

    pub fn zero(field: Rc<ZrField>)   -> Zr { Zr::new(Mpz::from(0), field) }
    pub fn one(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(1), field) }
    pub fn two(field: Rc<ZrField>)    -> Zr { Zr::new(Mpz::from(2), field) }
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

    // Tonelli-Shanks algorithm
    pub fn sqrt(&self, value: &Zr) -> Option<(Zr,Zr)> {
        // for better readability
        let n = value.value();
        let field = value.assume_field();
        let p = self.order();
 
        if ! self.legendre(n).is_one() {
            return None;
        }
        
        let mut q = p - 1;
        let mut s = Mpz::zero();
        while ((&q) & &Mpz::one()).is_zero() {
            s += 1;
            q >>= 1
        }
        
        if s.is_one() {
            let exp = (p+1)/4;
            let r1 = n.powm(&exp, p);
            let res2 = Zr{value: p - (&r1), field: Some(field.clone())};
            let res1 = Zr{value: r1,        field: Some(field.clone())};
            return Some((res1, res2));
        }
        
        let mut z = Mpz::from(2);
        while self.legendre(&z) != p-1 {
            z += 1
        }
        let mut c = z.powm(&q, p);
        
        let mut r = n.powm(&(((&q)+1)/2), p);
        let mut t = n.powm(&q, p);
        let mut m = s;
        
        loop {
            if t.is_one() {
                let res2 = Zr{value: p - &r, field: Some(field.clone())};
                let res1 = Zr{value: r,      field: Some(field.clone())};
                return Some((res1, res2));
            }
            
            let mut i = Mpz::zero();
            let mut z = t.clone();
            let mut b = c.clone();
            while !z.is_one() && &i < &(&m - 1) {
                z = (&z) * (&z) % p;
                i += 1;
            }
            let mut e = &m - &i - 1;
            while &e > &Mpz::zero() {
                b = (&b) * (&b) % p;
                e -= 1;
            }
            r = (&r) * (&b) % p;
            c = (&b) * (&b) % p;
            t = (&t) * (&c) % p;
            m = i;
        }
    }
}