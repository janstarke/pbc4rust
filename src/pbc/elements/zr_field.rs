use modpow::*;
use num_bigint_dig::*;
use std::ops;
use num_traits::{One, Zero};
use duplicate::duplicate;
use crate::pbc::elements::traits::*;
use std::ops::Neg;

#[derive(Debug, Clone]
pub struct ZrField {
    order: BigUint,
}

impl ZrField {
    fn two(order:&BigUint) -> Zr {
        Zr::new(BigUint::from(2), order.clone())
    }

    fn two_inverse(order:&BigUint) -> Zr {
        let two = BigUint::from(2);
        match two.mod_inverse(order) {
            Some(v) => Zr::new(v, order.clone()),
            None    => panic!("unable to invert")
        }
    }
}

#[duplicate(src_type; [BigUint]; [i32]; [u32]; [i64]; [u64];)]
impl From<src_type> for Zr {
    fn from(value: src_type) -> Self {
        Zr { value: BigUint::from(value), order: BigUint::zero() }
    }
}

impl One for ZrField { fn one() -> Zr { value: BigUInt::from(1), BigUInt::from(0) } }
impl Zero for Zr {
    fn zero() -> Self { Zr::from(0) }
    fn is_zero(&self) -> bool { self.value.is_zero() }
}

impl Neg for Zr {
    type Output = Zr;
    fn neg(self) -> Self::Output {
        if self.value.is_zero() {
            Zr::zero()
        } else {
            Zr::new(self.value.neg(), self.order.clone())
        }
    }
}

impl Square for Zr { fn square(&self) -> Self {self * self } }
impl Double for Zr { fn double(&self) -> Self {self * &Zr::two(&self.order) } }
impl Halve  for Zr { fn halve(&self)  -> Self {self * &Zr::two_inverse(&self.order) } }

// https://reinerm.wordpress.com/programming/rust/number-theory-in-rust/
fn power_mod(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let zero = BigUint::zero();
    if exp < &zero { unimplemented!() }
    let mut b = base % modulus;
    let mut result = 1;
    let mut e = exp.clone();
    while e > zero {
        if (e & BigUint::one()) != zero {
            result = (result * b) % modulus
        }
        b = (b * b) % modulus;
        e >>= 1
    }
    result % modulus;
}

// Legendre symbol, returns 1, 0, or -1 mod p
// https://reinerm.wordpress.com/programming/rust/number-theory-in-rust/
fn ls(a: &BigUint, p: &BigUint) -> BigUint {
    let exp = (p-BigUint::one()) / BigUint::from(2);
    power_mod(a, &exp, p)
}

fn is_sqrt(value: &BigUint, order: &BigUint) -> bool {
    a.is_zero() || ls(value, order) == BigUint::one()
}

// get some non quadratic residue
fn nqr(order: &BigUInt) -> Zr {
    let mut rng = rand::thread_rng();
    loop {
        let a = rng.gen_BigUint(BigUInt.one(), order);
        if is_sqrt(a, order) {
            return Zr{value: a, order: order.clone()}
        }
    }
}

// Tonelli-Shanks algorithm
// inspired by 
// https://reinerm.wordpress.com/programming/rust/number-theory-in-rust/
impl SquareRoot for Zr {
    type Item = Zr;
    fn sqrt(&self) -> Option<(Self,Self)> {
        // Arguments n, p as described in Wikipedia (WP)
 
        if ls(&self.value, &self.order) != BigUint::one() {
            // value is quadratic non-residue module order,
            // so there is no solution
            return None;
        }
        
        // WP step 1, factor out powers two.
        // variables Q, S named as at WP.
        let mut q = self.order - BigUint::one();
        let mut s = BigUint::zero();
        while (q & BigUint::one()).is_zero() {
            s += 1;
            q >>= 1
        }
        
        // WP step 1, direct solution
        if s.is_one() {
            let exp = (self.order+1)/BigUint::from(4);
            let r1 = power_mod(&self.value, &exp, &self.order);
            return Some((
                Zr{value: r1,              order: self.order},
                Zr{value: self.order - r1, order: self.order}
            ));
        }
        
        // WP step 2, select z, assign c
        let mut z = BigUint::from(2);
        while ls(&z, &self.order) != self.order-BigUint::one() {
            z += 1
        }
        let mut c = power_mod(&z, &q, &self.order);
        
        // WP step 3, assign R, t, M
        let mut r = power_mod(&self.value, (q+1)/2, &self.order);
        let mut t = power_mod(&self.value, &q, &self.order);
        let mut m = s;
        
        // WP step 4, loop
        loop {
            // WP step 4.1, termination condition
            if t.is_one() {
                return Some((
                    Zr{value: r,              order: self.order},
                    Zr{value: self.order - r, order: self.order}
                ));
            }
            
            // WP step 4.2, find lowest i...
            let mut i = BigUint::zero();
            let mut z = t;
            while !z.is_one() && i > BigUint::zero() {
                b = b * b % self.order;
                e -= 1
            }
            r = r * b % self.order;
            c = b * b % self.order; // more convenient to compute c before t
            t = t * c % self.order;
            m = i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::*;
    use crate::pbc::testlib::algebra::*;
    use crate::test_one;
    use crate::test_zero;
    use crate::test_associativity;
    use crate::test_commutativity;
    use crate::test_double_and_halve;
    use crate::test_distributivity;
    use crate::test_square_and_sqrt;

    fn a() -> Zr {Zr::from_u64(VALUE_A, ORDER)}
    fn b() -> Zr {Zr::from_u64(VALUE_B, ORDER)}
    fn c() -> Zr {Zr::from_u64(VALUE_C, ORDER)}
    fn d() -> Zr {Zr::from_u64(VALUE_D, ORDER)}
    
    test_one!(Zr, a());
    test_zero!(Zr, a());
    test_double_and_halve!(Zr, a());
    test_square_and_sqrt!(Zr, a());
    test_commutativity!(Zr, add, a(), b());
    test_commutativity!(Zr, mul, a(), b());
    test_associativity!(Zr, add, a(), b(), c());
    test_associativity!(Zr, mul, a(), b(), c());
    test_distributivity!(Zr, add, mul, d(), a(), b());
}