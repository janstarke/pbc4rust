use crate::pbc::{Z, Halve};
use crate::pbc::Double;
use num_traits::{One, Zero};

#[test]
fn test_z_mul_commutativity() {
    let a: Z = Z::from(12345 as i32);
    let c: Z = Z::from(6789 as i32);
    assert_eq!(&a * &c, &c * &a);
}

#[test]
fn test_z_mul_one() {
    let a: Z = Z::from(12345 as i32);
    assert_eq!(&a * &Z::one(), a);
    assert_eq!(&Z::one() * &a, a);
}

#[test]
fn test_z_add_commutativity() {
    let a: Z = Z::from(12345 as i32);
    let c: Z = Z::from(6789 as i32);
    assert_eq!(&a + &c, &c + &a);
}

#[test]
fn test_z_add_zero() {
    let a: Z = Z::from(12345 as i32);
    assert_eq!(&a + &Z::zero(), a);
    assert_eq!(&Z::zero() + &a, a);
}

#[test]
fn test_z_add_associativity() {
    let a: Z = Z::from(1234 as i32);
    let b: Z = Z::from(3456 as i32);
    let c: Z = Z::from(91 as i32);
    let d: Z = Z::from(1234 + 3456 + 91 as i32);
    assert_eq!(&a + &(&b + &c), d);
    assert_eq!(&(&a + &b) + &c, d);
}

#[test]
fn test_z_mul_associativity() {
    let a: Z = Z::from(1234 as i32);
    let b: Z = Z::from(3456 as i32);
    let c: Z = Z::from(91 as i32);
    let d: Z = Z::from(1234 * 3456 * 91 as i32);
    assert_eq!(&a * &(&b * &c), d);
    assert_eq!(&(&a * &b) * &c, d);
}

#[test]
fn test_z_distributivity() {
    let a: Z = Z::from(1234 as i32);
    let b: Z = Z::from(3456 as i32);
    let c: Z = Z::from(91 as i32);
    let d: Z = Z::from(91 * (1234 + 3456) as i32);
    assert_eq!(&c * &(&a + &b), d);
    assert_eq!(&(&c * &a) + &(&c * &b), d);
}

#[test]
fn test_z_double_and_halve() {
    let a: Z = Z::from(1234 as i32);
    let b = a.double();
    assert_ne!(&a, &b);

    let c = b.halve();
    assert_eq!(&a, &c);
}
