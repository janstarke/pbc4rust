use gmp::mpz::Mpz;
use std::ops;
use num_traits::{One, Zero};
//use duplicate::duplicate;
use crate::pbc::elements::traits::*;
use std::ops::Neg;
use std::rc::Rc;
use super::ZrField;

#[derive(Debug, Clone, PartialEq)]
pub enum Zr {
    Zero(Mpz,  Option<Rc<ZrField>>),
    One(Mpz,   Option<Rc<ZrField>>),
    Other(Mpz,        Rc<ZrField>)
}

impl Zr {
    pub fn new(value: Mpz, field: Rc<ZrField>) -> Zr {
        let value = value % field.order();
        if value.is_zero() {
            Zr::Zero(value, Some(field))
        } else if value.is_one() {
            Zr::One(value, Some(field))
        } else {
            Zr::Other(value, field)
        }
    }

    pub fn value(&self) -> &Mpz {
        match self {
            Zr::Zero(v, _) => &v,
            Zr::One(v, _)  => &v,
            Zr::Other(v, _) => &v
        }
    }

    pub fn field(&self) -> Option<Rc<ZrField>> {
        match self {
            Zr::Zero(_, None)       => None,
            Zr::One(_, None)        => None,
            Zr::Zero(_, Some(f))    => Some(f.clone()),
            Zr::One(_, Some(f))     => Some(f.clone()),
            Zr::Other(_, f)         => Some(f.clone()),
        }
    }

    fn common_field(z1: &Zr, z2: &Zr) -> Option<Rc<ZrField>> {
        match (z1, z2) {
            (Zr::Zero(_,None), Zr::Zero(_,None))                 => None,
            (Zr::Zero(_,Some(f1)), Zr::Zero(_,None))             => Some(f1.clone()),
            (Zr::Zero(_,None), Zr::Zero(_,Some(f2)))             => Some(f2.clone()),

            (Zr::Zero(_,None), Zr::One(_,None))                  => None,
            (Zr::Zero(_,Some(f1)), Zr::One(_,None))              => Some(f1.clone()),
            (Zr::Zero(_,None), Zr::One(_,Some(f2)))              => Some(f2.clone()),

            (Zr::One(_,None),  Zr::Zero(_,None))                 => None,
            (Zr::One(_,Some(f1)), Zr::Zero(_,None))              => Some(f1.clone()),
            (Zr::One(_,None), Zr::Zero(_,Some(f2)))              => Some(f2.clone()),

            (Zr::One(_,None),  Zr::One(_,None))                  => None,
            (Zr::One(_,Some(f1)), Zr::One(_,None))               => Some(f1.clone()),
            (Zr::One(_,None), Zr::One(_,Some(f2)))               => Some(f2.clone()),

            (Zr::Zero(_,None), Zr::Other(_, f2))                 => Some(f2.clone()),
            (Zr::Zero(_,Some(f1)), Zr::Other(_, f2)) if f1 == f2 => Some(f1.clone()),

            (Zr::One(_,None),  Zr::Other(_, f2))                 => Some(f2.clone()),
            (Zr::One(_,Some(f1)), Zr::Other(_, f2)) if f1 == f2  => Some(f1.clone()),

            (Zr::Other(_, f1), Zr::Zero(_,None))                 => Some(f1.clone()),
            (Zr::Other(_, f1), Zr::Zero(_,Some(f2))) if f1 == f2 => Some(f1.clone()),

            (Zr::Other(_, f1), Zr::One(_,None))                  => Some(f1.clone()),
            (Zr::Other(_, f1), Zr::One(_,Some(f2))) if f1 == f2  => Some(f1.clone()),

            (Zr::Other(_, f1), Zr::Other(_, f2)) if f1 == f2     => Some(f1.clone()),
            (_,_)                                                => None
        }
    }

    pub fn legendre(&self) -> Mpz {
        match self {
            Zr::Zero(_,_)     => Mpz::from(0),
            Zr::One(_,_)      => Mpz::from(1),
            Zr::Other(e, f) => f.legendre(&e)
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
impl One for Zr {
    fn one() -> Self { Zr::One(Mpz::from(1), None) } 
    fn is_one(&self) -> bool {
        match self {
            Zr::One(_, _)  => true,
            _           => false
        }
    }
}
impl Zero for Zr {
    fn zero() -> Self { Zr::One(Mpz::from(0), None) }
    fn is_zero(&self) -> bool {
        match self {
            Zr::Zero(_, _) => true,
            _           => false
        }
    }
}

macro_rules! add_operators {
    ($($op:tt)+) => {
        $(
            impl_op!($op |lhs:Zr, rhs:Zr | -> Zr {
                let field = Zr::common_field(&lhs, &rhs).expect("unable to calculate");
                Zr::new(lhs.value() $op rhs.value(), field)
            });

            impl_op!($op |lhs:&Zr, rhs:&Zr | -> Zr {
                let field = Zr::common_field(lhs, rhs).expect("unable to calculate");
                Zr::new(lhs.value() $op rhs.value(), field)
            });
        )+
    };
}
add_operators!(+-*);

impl_op!(/ |lhs:&Zr, rhs:&Zr | -> Zr {
    let field = Zr::common_field(lhs, rhs).expect("unable to calculate");
    Zr::new(lhs.value() * &field.inverse_of(&rhs.value()), field)
});

impl_op!(/ |lhs:Zr, rhs:Zr | -> Zr {
    let field = Zr::common_field(&lhs, &rhs).expect("unable to calculate");
    Zr::new(lhs.value() * &field.inverse_of(&rhs.value()), field)
});


impl Neg for Zr {
    type Output = Zr;
    fn neg(self) -> Self::Output {  
        match self {
            Zr::Zero(_, _)      => self.clone(),
            Zr::One(_, Some(f)) => Zr::new(f.order() - Mpz::one(), f.clone()),
            Zr::Other(e, f)     => Zr::new(f.order() - e, f),
            _                   => panic!("unable to negate"),
        }
    }
}

impl Square for Zr { fn square(&self) -> Self {self * self } }
impl Double for Zr {
    fn double(&self) -> Self {
        match self {
            Zr::Zero(_, _)          => self.clone(),
            Zr::One(e, Some(f))     => Zr::new(e * Mpz::from(2), f.clone()),
            Zr::Other(e, f)         => Zr::new(e * Mpz::from(2), f.clone()),
            _                       => panic!("unable to double"),
        }
    }
}
impl Halve  for Zr {
    fn halve(&self) -> Self {
        match self {
            Zr::Zero(_, _)          => self.clone(),
            Zr::One(e, Some(f))     => Zr::new(e * &ZrField::two_inverse(f.as_ref()), f.clone()),
            Zr::Other(e, f)         => Zr::new(e * &ZrField::two_inverse(f.as_ref()), f.clone()),
            _                       => panic!("unable to double"),
        }
    }
}

impl Zr {
    pub fn is_sqrt(&self) -> bool {
        match self {
            Zr::Zero(_, _) => true,
            _              => self.legendre() == Mpz::one(),
        }
    }
}

impl Sqrt for Zr {
    type Item = Zr;
    fn sqrt(&self) -> Option<(Self,Self)> {
        match self.field() {
            None        => None,
            Some(f)     => Zr::sqrt(f, self.value()),
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
}