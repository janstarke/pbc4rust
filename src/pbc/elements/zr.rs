//use rand::*;
use gmp::mpz::Mpz;
//use gmp::rand::RandState;
use std::ops;
use num_traits::{One, Zero};
use duplicate::duplicate;
use crate::pbc::elements::traits::*;
use std::ops::Neg;

#[derive(Debug, Clone, PartialEq)]
pub struct Zr {
    value: Mpz,
    order: Mpz,
}

impl Zr {

    pub fn new (value: Mpz, order: Mpz) -> Zr {
        Zr { value: value, order: order }
    }

    pub fn from_u64(value: u64, order: u64) -> Zr {
        Zr {
            value: Mpz::from(value),
            order: Mpz::from(order)
        }
    }

    pub fn common_order(me: &Zr, other: &Zr) -> Mpz {
        // 0 should be no valid order,
        // so we use this to mark special values, such as 0 or 1
        match (&me.order, &other.order) {
            (lhs, rhs) if lhs == rhs => lhs.clone(),
            (lhs, rhs) if lhs == &Mpz::zero() => rhs.clone(),
            (lhs, rhs) if rhs == &Mpz::zero() => lhs.clone(),
            _ => panic!("values of different orders"),
        }
    }

    fn two(order:&Mpz) -> Zr {
        Zr::new(Mpz::from(2 as u32), order.clone())
    }

    fn two_inverse(order:&Mpz) -> Zr {
        let two: Mpz = Mpz::from(2 as u32);
        match two.invert(order) {
            Some(v) => Zr::new(v, order.clone()),
            None    => panic!("unable to invert")
        }
    }
}

#[duplicate(src_type; [Mpz]; [u32]; [u64];)]
impl From<src_type> for Zr {
    fn from(value: src_type) -> Self {
        Zr { value: Mpz::from(value), order: Mpz::zero() }
    }
}

macro_rules! add_operators {
    ($($op:tt)+) => {
        $(
            impl_op!($op |lhs:Zr, rhs: Zr| -> Zr {
                let order = Zr::common_order(&lhs, &rhs);
                Zr::new ((&lhs.value $op &rhs.value) % &order, order)
            });
            impl_op!($op |lhs:&Zr, rhs:&Zr | -> Zr {
                let order = Zr::common_order(&lhs, &rhs);
                Zr::new ((&lhs.value $op &rhs.value) % &order, order)
            });
        )+
    };
}
add_operators!(+-*);

impl_op!(* |lhs:Zr, rhs:u64 | -> Zr {
    assert_ne!(lhs.order, Mpz::zero());
    Zr::new ((&lhs.value * rhs) % &lhs.order, lhs.order.clone())
});

impl_op!(/ |lhs:Zr, rhs:Zr | -> Zr {
    let order = Zr::common_order(&lhs, &rhs);
    match rhs.value.invert(&order) {
        Some(v) => Zr::new (&lhs.value * &v, order),
        None    => panic!("unable to invert")
    }
});

impl One for Zr { fn one() -> Self { Zr::from(1 as u32) } }
impl Zero for Zr {
    fn zero() -> Self { Zr::from(0 as u32) }
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

// Legendre symbol, returns 1, 0, or -1 mod p
fn ls(a: &Mpz, p: &Mpz) -> Mpz {
    let exp = (p-Mpz::one()) / Mpz::from(2 as u32);
    a.powm(&exp, p)
}
/*
fn is_sqrt(value: &Mpz, order: &Mpz) -> bool {
    value.is_zero() || ls(value, order) == Mpz::one()
}

// get some quadratic nonresidue
fn nqr(order: &Mpz) -> Zr {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = RandState::new();
    rng2.seed(Mpz::from(rng1.next_u64()));
    loop {
        let a = rng2.urandom(order);
        if a > Mpz::one() {
            if is_sqrt(&a, order) {
                return Zr{value: a, order: order.clone()}
            }
        }
    }
}
*/
// Tonelli-Shanks algorithm
impl SquareRoot for Zr {
    type Item = Zr;
    fn sqrt(&self) -> Option<(Self,Self)> {
        // Arguments n, p as described in Wikipedia (WP)
 
        if ls(&self.value, &self.order) != Mpz::one() {
            // value is quadratic non-residue module order,
            // so there is no solution
            return None;
        }
        
        // WP step 1, factor out powers two.
        // variables Q, S named as at WP.
        let mut q = &(self.order) - Mpz::one();
        let mut s = Mpz::zero();
        while ((&q) & &Mpz::one()).is_zero() {
            s += 1;
            q >>= 1
        }
        
        // WP step 1, direct solution
        if s.is_one() {
            let exp = ((&self.order)+1)/Mpz::from(4);
            let r1 = self.value.powm(&exp, &self.order);
            let res2 = Zr{value: (&self.order) - (&r1), order: self.order.clone()};
            let res1 = Zr{value: r1,              order: self.order.clone()};
            return Some((res1, res2));
        }
        
        // WP step 2, select z, assign c
        let mut z = Mpz::from(2);
        while ls(&z, &self.order) != (&self.order)-&Mpz::one() {
            z += 1
        }
        let mut c = z.powm(&q, &self.order);
        
        // WP step 3, assign R, t, M
        let mut r = self.value.powm(&(((&q)+1)/2), &self.order);
        let mut t = self.value.powm(&q, &self.order);
        let mut m = s;
        
        // WP step 4, loop
        loop {
            // WP step 4.1, termination condition
            if t.is_one() {
                let res2 = Zr{value: &self.order - &r, order: self.order.clone()};
                let res1 = Zr{value: r,                order: self.order.clone()};
                return Some((res1, res2));
            }
            
            // WP step 4.2, find lowest i...
            let mut i = Mpz::zero();
            let mut z = t.clone();
            let mut b = c.clone();
            while !z.is_one() && &i < &(&m - 1) {
                z = (&z) * (&z) % &(self.order);
                i += 1;
            }
            let mut e = &m - &i - 1;
            while &e > &Mpz::zero() {
                b = (&b) * (&b) % &(self.order);
                e -= 1;
            }
            r = (&r) * (&b) % &self.order;
            c = (&b) * (&b) % &self.order; // more convenient to compute c before t
            t = (&t) * (&c) % &self.order;
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