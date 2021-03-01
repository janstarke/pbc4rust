use gmp::mpz::*;
use std::ops;
use num_traits::{Signed, Num, One, Zero};
use gmp::sign::Sign;
use duplicate::duplicate;
use crate::pbc::elements::traits::*;
use std::ops::Neg;

#[derive(Debug, Clone, PartialEq)]
pub struct Z {
    value: Mpz
}

impl Z {
    pub fn new(d: Mpz) -> Z {
        Z { value: d }
    }
}

/// takes ownership of `op`
impl From<Mpz> for Z {
    fn from(op: Mpz) -> Self {
        Self {
            value: op
        }
    }
}

/// creates a copy of `op`
impl<'a> From<&'a Mpz> for Z {
    fn from(op: &'a Mpz) -> Self {
        Self {
            value: op.clone()
        }
    }
}

#[duplicate(int_type; [i32]; [i64]; [u32]; [u64]; )]
impl From<int_type> for Z { fn from(op: int_type) -> Self { Self::from(Mpz::from(op)) } }

impl Num for Z {
    type FromStrRadixErr = ParseMpzError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match Mpz::from_str_radix(str, radix as u8){
            Ok(value) => Ok(Z::from(value)),
            Err(why) => Err(why)
        }
    }
}

macro_rules! add_operators {
    ($($op:tt)+) => {
        $(
            impl_op!($op |lhs:Z, rhs: Z| -> Z {Z::new (&lhs.value $op &rhs.value)});
            impl_op!($op |lhs:&Z, rhs:&Z | -> Z {Z::new (&lhs.value $op &rhs.value)});
        )+
    };
}
add_operators!(+-*/%);
impl_op!(* |lhs:Z, rhs:i64 | -> Z {Z::new (&lhs.value * rhs)});
impl_op!(* |lhs:Z, rhs:u64 | -> Z {Z::new (&lhs.value * rhs)});

impl One for Z {
    fn one() -> Self { Z { value: Mpz::one() } }
}

impl Zero for Z {
    fn zero() -> Self { Z { value: Mpz::zero() } }
    fn is_zero(&self) -> bool { self.value.is_zero() }
}

impl Neg for Z {
    type Output = Z;
    fn neg(self) -> Self::Output { Z { value: self.value.neg() } }
}

impl Signed for Z {
    fn abs(&self) -> Self { Z { value: self.value.abs() } }

    fn abs_sub(&self, rhs: &Self) -> Self {
        let d = &self.value - &rhs.value;
        if d < Mpz::zero() {
            Self::zero()
        } else {
            Z { value: d }
        }
    }

    fn signum(&self) -> Self {
        match self.value.sign() {
            Sign::Negative => Self::from(-1),
            Sign::Zero => Self::zero(),
            Sign::Positive => Self::one()
        }
    }

    fn is_positive(&self) -> bool { self.value.gt(&Mpz::zero()) }
    fn is_negative(&self) -> bool { self.value.lt(&Mpz::zero()) }
}

impl Square     for Z { fn square(&self) -> Self {self * self} }
impl Double     for Z { fn double(&self) -> Self {Self {value: &self.value << 1 } } }
impl Halve      for Z { fn halve(&self)  -> Self {Self {value: &self.value >> 1 } } }
impl Sqrt for Z {
    type Item = Z;
    fn sqrt(&self) -> Option<(Self,Self)> {
        let s1 = self.value.sqrt();
        let s2 = - &s1;
        Some((
            Self {value: s1},
            Self {value: s2}
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pbc::elements::ZField;
    use std::rc::Rc;
    use std::ops::*;
    use crate::pbc::testlib::algebra::*;
    use crate::test_one;
    use crate::test_zero;
    use crate::test_associativity;
    use crate::test_commutativity;
    use crate::test_double_and_halve;
    use crate::test_distributivity;
    use crate::test_square_and_sqrt;

    fn field() -> Rc<ZField> { Rc::new(ZField::new()) }
    
    test_one!(Z, ZField, field());
    test_zero!(Z, ZField, field());
    test_double_and_halve!(Z, ZField, field());
    test_square_and_sqrt!(Z, ZField, field());
    test_commutativity!(Z, add, ZField, field());
    test_commutativity!(Z, mul, ZField, field());
    test_associativity!(Z, add, ZField, field());
    test_associativity!(Z, mul, ZField, field());
    test_distributivity!(Z, add, mul, ZField, field());
}