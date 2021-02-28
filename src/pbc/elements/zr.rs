//use rand::*;
use gmp::mpz::Mpz;
//use gmp::rand::RandState;
use std::ops;
use num_traits::{One, Zero};
use duplicate::duplicate;
use crate::pbc::elements::traits::*;
use std::ops::Neg;
use std::rc::Rc;
use super::ZrField;

#[derive(Debug, Clone, PartialEq)]
pub struct Zr {
    pub value: Mpz,
    pub field: Option<Rc<ZrField>>,
}

impl Zr {
    pub fn new(value: Mpz, field: Rc<ZrField>) -> Zr {
        Zr {
            value: value,
            field: Some(field)
        }
    }

    fn common_field(z1: &Zr, z2: &Zr) -> Option<Rc<ZrField>> {
        match (z1.field.as_ref(), z2.field.as_ref()) {
            (None, None)                     => None,
            (Some(v), None)                  => Some(v.clone()),
            (None, Some(v))                  => Some(v.clone()),
            (Some(v1), Some(v2)) if v1 == v2 => Some(v1.clone()),
            (_, _)                           => None
        }
    }

    pub fn assume_field(&self) -> Rc<ZrField> {
        self.field.as_ref().expect("missing field").clone()
    }

    pub fn order(&self) -> Rc<Mpz> {
        self.assume_field().order()
    }

    pub fn from_u64(value: u64, field: Rc<ZrField>) -> Zr {
        Zr {
            value: Mpz::from(value),
            field: Some(field)
        }
    }

    pub fn two(field: Rc<ZrField>) -> Zr {
        Zr {
            value: Mpz::from(2 as u32),
            field: Some(field)
        }
    }

    pub fn two_inverse(field: Rc<ZrField>) -> Zr {
        let two: Mpz = Mpz::from(2 as u32);
        match two.invert(field.order().as_ref()) {
            Some(v) => Zr{value: v, field: Some(field)},
            None    => panic!("unable to invert")
        }
    }
}
impl One for Zr { fn one() -> Self { Zr { value: Mpz::from(1), field: None} } }
impl Zero for Zr {
    fn zero() -> Self { Zr { value: Mpz::from(0), field: None} }
    fn is_zero(&self) -> bool { self.value.is_zero() }
}

#[duplicate(src_type; [Mpz]; [u32]; [u64]; [i32]; [i64])]
impl From<src_type> for Zr {
    fn from(value: src_type) -> Self {
        Zr { value: Mpz::from(value), field: None }
    }
}

/*
macro_rules! common_order {
    ($me: expr, $other: expr) => {
        Zr::common_field($me, $other).map(|v| v.order())
    };
}
*/
/*
impl Zr {

    pub fn from_u64(value: u64, order: u64) -> Zr {
        Zr {
            value: Mpz::from(value),
            order: Mpz::from(order)
        }
    }

}

#[duplicate(src_type; [Mpz]; [u32]; [u64];)]
impl From<src_type> for Zr {
    fn from(value: src_type) -> Self {
        Zr { value: Mpz::from(value), order: Mpz::zero() }
    }
}
*/

macro_rules! add_operators {
    ($($op:tt)+) => {
        $(
            impl_op!($op |lhs:Zr, rhs:Zr | -> Zr {
                let field = Zr::common_field(&lhs, &rhs).expect("unable to calculate");
                Zr {
                    value: (&lhs.value $op &rhs.value) % field.order().as_ref(),
                    field: Some(field)
                }
            });

            impl_op!($op |lhs:&Zr, rhs:&Zr | -> Zr {
                let field = Zr::common_field(lhs, rhs).expect("unable to calculate");
                let order = field.order();
                Zr {
                    value: (&lhs.value $op &rhs.value) % field.order().as_ref(),
                    field: Some(field)
                }
            });
        )+
    };
}
add_operators!(+-*);

impl_op!(/ |lhs:&Zr, rhs:&Zr | -> Zr {
    let field = Zr::common_field(lhs, rhs).expect("unable to calculate");
    match rhs.value.invert(field.order().as_ref()) {
        Some(v) => Zr {value: &lhs.value * &v, field: Some(field)},
        None    => panic!("unable to invert")
    }
});

impl_op!(/ |lhs:Zr, rhs:Zr | -> Zr {
    let field = Zr::common_field(&lhs, &rhs).expect("unable to calculate");
    let order = field.order();
    match rhs.value.invert(field.order().as_ref()) {
        Some(v) => Zr {value: &lhs.value * &v, field: Some(field)},
        None    => panic!("unable to invert")
    }
});


impl Neg for Zr {
    type Output = Zr;
    fn neg(self) -> Self::Output {
        if self.value.is_zero() {
            self
        } else {
            Zr {
                value: self.value.neg(),
                field: self.field
            }
        }
    }
}

impl Square for Zr { fn square(&self) -> Self {self * self } }
impl Double for Zr { fn double(&self) -> Self {self * &Zr::two(self.assume_field()) } }
impl Halve  for Zr { fn halve(&self)  -> Self {self * &Zr::two_inverse(self.assume_field()) } }

// Legendre symbol, returns 1, 0, or -1 mod p
fn ls(a: &Mpz, p: &Mpz) -> Mpz {
    let exp = (p-Mpz::one()) / Mpz::from(2 as u32);
    a.powm(&exp, p)
}

impl Zr {
    fn is_sqrt(&self) -> bool {
        self.value.is_zero() || ls(&self.value, self.assume_field().order().as_ref()) == Mpz::one()
    }
}
/*
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
        // for better readability
        let n = &self.value;
        let field = self.assume_field();
        let pr = self.order();
        let p = pr.as_ref();
 
        if ! ls(n, p).is_one() {
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
            let r1 = self.value.powm(&exp, p);
            let res2 = Zr{value: p - (&r1), field: Some(field.clone())};
            let res1 = Zr{value: r1,        field: Some(field.clone())};
            return Some((res1, res2));
        }
        
        let mut z = Mpz::from(2);
        while ls(&z, p) != p-1 {
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

    fn a() -> Zr {Zr::from_u64(VALUE_A, field())}
    fn b() -> Zr {Zr::from_u64(VALUE_B, field())}
    fn c() -> Zr {Zr::from_u64(VALUE_C, field())}
    fn d() -> Zr {Zr::from_u64(VALUE_D, field())}
    
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