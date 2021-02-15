use gmp::mpz::*;
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
        Zr::new(Mpz::from(2), order.clone())
    }

    fn two_inverse(order:&Mpz) -> Zr {
        let two = Mpz::from(2);
        match two.invert(&order) {
            Some(v) => Zr::new(v, order.clone()),
            None    => panic!("unable to invert")
        }
    }
}

#[duplicate(src_type; [Mpz]; [i32]; [u32]; [i64]; [u64];)]
impl From<src_type> for Zr {
    fn from(value: src_type) -> Self {
        Zr { value: Mpz::from(value), order: Mpz::zero() }
    }
}


/*
impl Num for Zr {
    type FromStrRadixErr = ParseMpzError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match Mpz::from_str_radix(str, radix as u8){
            Ok(value) => Ok(Z::from(value)),
            Err(why) => Err(why)
        }
    }
}
*/
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

#[duplicate(int_type; [i64]; [u64];)]
impl_op!(* |lhs:Zr, rhs:int_type | -> Zr {
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

impl One for Zr { fn one() -> Self { Zr::from(1) } }
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

    fn a() -> Zr {Zr::from_u64(VALUE_A, ORDER)}
    fn b() -> Zr {Zr::from_u64(VALUE_B, ORDER)}
    fn c() -> Zr {Zr::from_u64(VALUE_C, ORDER)}
    fn d() -> Zr {Zr::from_u64(VALUE_D, ORDER)}
    
    test_one!(Zr, a());
    test_zero!(Zr, a());
    test_double_and_halve!(Zr, a());
    test_commutativity!(Zr, add, a(), b());
    test_commutativity!(Zr, mul, a(), b());
    test_associativity!(Zr, add, a(), b(), c());
    test_associativity!(Zr, mul, a(), b(), c());
    test_distributivity!(Zr, add, mul, d(), a(), b());
}