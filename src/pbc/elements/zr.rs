use gmp::mpz::Mpz;
use std::ops;
use num_traits::{One, Zero};
use crate::pbc::elements::traits::*;
use std::ops::Neg;
use std::rc::Rc;
use super::ZrField;

#[derive(Debug, Clone, PartialEq)]
pub struct Zr {
    value: Mpz,
    field: Rc<ZrField>
}

impl Element<AtomicElement> for Zr {
    type FieldType = ZrField;

    fn field(&self) -> Rc<Self::FieldType> {
        Rc::clone(&self.field)
    }

    fn double(&self) -> Self {
        Zr::new(
            self.value() * Mpz::from(2),
            Rc::clone(&self.field)
        )
    }

    fn halve(&self) -> Self {
        Zr::new(
            self.value() * &Self::FieldType::two_inverse(self.field.as_ref()),
            Rc::clone(&self.field)
        )
    }

    fn square(&self) -> Self {self * self }

    fn is_sqrt(&self) -> bool {
        if self.is_zero() {
            true
        } else {
            self.legendre() == Mpz::one()
        }
    }

    fn sqrt(&self) -> Option<(Self,Self)> {
        Zr::sqrt(self.field(), self.value())
    }
}


impl<'b> Set<&'b Zr> for Zr {
    fn set(mut self, other: &'b Self) -> Self {
        assert_eq!(self.field(), other.field());
        self.value = other.value.clone();
        self
    }
}

impl Zr {
    pub fn new(value: Mpz, field: Rc<ZrField>) -> Zr {
        let value = value % field.order();
        Self {
            value,
            field
        }
    }

    pub fn value(&self) -> &Mpz {
        &self.value
    }

    fn common_field(z1: &Zr, z2: &Zr) -> Option<Rc<ZrField>> {
        if z1.field == z2.field {
            Some(Rc::clone(&z1.field))
        } else {
            None
        }
    }

    pub fn legendre(&self) -> Mpz {
        if self.is_zero() {
            Mpz::from(0)
        } else if self.is_one() {
            Mpz::from(1)
        } else {
            self.field.legendre(&self.value())
        }
    }

    // Tonelli-Shanks algorithm
    pub fn sqrt(field: Rc<ZrField>, n: &Mpz) -> Option<(Zr,Zr)> {
        // for better readability
        let p = field.order();
 
        if ! field.legendre(n).is_one() {
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
            let res2 = Zr::new(p - (&r1), field.clone());
            let res1 = Zr::new(r1,        field);
            return Some((res1, res2));
        }
        
        let mut z = Mpz::from(2);
        while field.legendre(&z) != p-1 {
            z += 1
        }
        let mut c = z.powm(&q, p);
        
        let mut r = n.powm(&(((&q)+1)/2), p);
        let mut t = n.powm(&q, p);
        let mut m = s;
        
        loop {
            if t.is_one() {
                let res2 = Zr::new(p - &r, field.clone());
                let res1 = Zr::new(r,      field);
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

impl CanBeOne for Zr {
    fn is_one(&self) -> bool { self.value.is_one() }
}

impl CanBeZero for Zr {
    fn is_zero(&self) -> bool { self.value.is_zero() }
}

macro_rules! add_operators {
    ($($op:tt)+) => {
        $(
            impl_op_ex!($op |lhs:&Zr, rhs:&Zr | -> Zr {
                let field = Zr::common_field(&lhs, &rhs).unwrap_or_else(|| panic!("unable to calculate, because fields for lhs and rhs are different (lhs ∈ {{0..{}, nqr={:?}}}, rhs ∈ {{0..{}, nqr={:?}}})", lhs.field.order(), ZrField::nqr(Rc::clone(&lhs.field)), rhs.field.order(), ZrField::nqr(Rc::clone(&rhs.field))));
                Zr::new(lhs.value() $op rhs.value(), field)
            });
        )+
    };
}
add_operators!(+-*);

impl_op_ex!(/ |lhs:&Zr, rhs:&Zr | -> Zr {
    let field = Zr::common_field(lhs, rhs).expect("unable to calculate");
    Zr::new(lhs.value() * &field.inverse_of(&rhs.value()), field)
});


impl Neg for Zr {
    type Output = Zr;
    fn neg(self) -> Self::Output {
        Zr::new(self.field.order() - Mpz::one(), Rc::clone(&self.field))
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
    use crate::test_nqr;

    fn field() -> Rc<ZrField> { Rc::new(ZrField::new(Mpz::from(ORDER))) }
    
    test_one!(Zr, ZrField, field());
    test_zero!(Zr, ZrField, field());
    test_double_and_halve!(Zr, ZrField, field());
    test_square_and_sqrt!(Zr, ZrField, field());
    test_commutativity!(Zr, add, ZrField, field());
    test_commutativity!(Zr, mul, ZrField, field());
    test_associativity!(Zr, add, ZrField, field());
    test_associativity!(Zr, mul, ZrField, field());
    test_distributivity!(Zr, add, mul, ZrField, field());
    test_nqr!(Zr, ZrField, field());
}