use gmp::mpz::*;
use std::ops;
use num_traits::{Signed, Num, One, Zero};
use gmp::sign::Sign;
use duplicate::duplicate;
use crate::pbc::elements::traits::*;
use std::ops::Neg;

// Z ist just a wrapper for Mpz
#[derive(Debug, Clone, PartialEq)]
pub struct Z {
    data: Mpz,
}

impl Z {
    pub fn new(d: Mpz) -> Z {
        Z { data: d }
    }
}

/// takes ownership of `op`
impl From<Mpz> for Z {
    fn from(op: Mpz) -> Self {
        Self {
            data: op
        }
    }
}

/// creates a copy of `op`
impl<'a> From<&'a Mpz> for Z {
    fn from(op: &'a Mpz) -> Self {
        Self {
            data: op.clone()
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

impl_op!(+ |lhs:Z, rhs: Z| -> Z {Z::new (&lhs.data + &rhs.data)});
impl_op!(+ |lhs:&Z, rhs:&Z | -> Z {Z::new (&lhs.data + &rhs.data)});
impl_op!(- |lhs:Z, rhs: Z| -> Z {Z::new (&lhs.data - &rhs.data)});
impl_op!(- |lhs:&Z, rhs: &Z| -> Z {Z::new (&lhs.data - &rhs.data)});
impl_op!(* |lhs:Z, rhs: Z| -> Z {Z::new (&lhs.data * &rhs.data)});
impl_op!(* |lhs:&Z, rhs: &Z| -> Z {Z::new (&lhs.data * &rhs.data)});
impl_op!(/ |lhs:Z, rhs: Z| -> Z {Z::new (&lhs.data / &rhs.data)});
impl_op!(/ |lhs:&Z, rhs: &Z| -> Z {Z::new (&lhs.data / &rhs.data)});
impl_op!(% |lhs:Z, rhs: Z| -> Z {Z::new (&lhs.data % &rhs.data)});
impl_op!(% |lhs:&Z, rhs: &Z| -> Z {Z::new (&lhs.data % &rhs.data)});


impl ops::Mul<i64> for Z {
    type Output = Z;
    fn mul(self, rhs: int_type) -> Self::Output { Z { data: &self.data * (rhs) } }
}

impl ops::Mul<u64> for Z {
    type Output = Z;
    fn mul(self, rhs: int_type) -> Self::Output { Z { data: &self.data * (rhs) } }
}

impl One for Z {
    fn one() -> Self { Z { data: Mpz::one() } }
}

impl Zero for Z {
    fn zero() -> Self { Z { data: Mpz::zero() } }
    fn is_zero(&self) -> bool { self.data.is_zero() }
}

impl Neg for Z {
    type Output = Z;
    fn neg(self) -> Self::Output { Z { data: self.data.neg() } }
}

impl Signed for Z {
    fn abs(&self) -> Self { Z { data: self.data.abs() } }

    fn abs_sub(&self, rhs: &Self) -> Self {
        let d = &self.data - &rhs.data;
        if d < Mpz::zero() {
            Self::zero()
        } else {
            Z { data: d }
        }
    }

    fn signum(&self) -> Self {
        match self.data.sign() {
            Sign::Negative => Self::from(-1),
            Sign::Zero => Self::zero(),
            Sign::Positive => Self::one()
        }
    }

    fn is_positive(&self) -> bool { self.data.gt(&Mpz::zero()) }
    fn is_negative(&self) -> bool { self.data.lt(&Mpz::zero()) }
}

impl Square for Z { fn square(&self) -> Self {self * self} }
impl Double for Z { fn double(&self) -> Self {Self {data: &self.data << 1 } } }
impl Halve  for Z { fn halve(&self)  -> Self {Self {data: &self.data >> 1 } } }
