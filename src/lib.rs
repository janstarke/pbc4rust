mod pbc;

#[cfg(test)]
mod tests {
    use crate::pbc::Z;
    use num_traits::{One, Zero};

    #[test]
    fn test_z_commutativity() {
        let a: Z = Z::from(12345 as i32);
        let b: Z = Z::from(12345 as i32);
        assert_eq!(a, b);
        assert_eq!(&a + &Z::zero(), b);
        assert_eq!(&Z::zero() + &a, b);
        assert_eq!(&a * &Z::one(), b);
        assert_eq!(&Z::one() * &a, b);
    }
}
