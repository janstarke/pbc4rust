pub use concat_idents::concat_idents;

pub const VALUE_A:u64 = 1234;
pub const VALUE_B:u64 = 3456;
pub const VALUE_C:u64 = 6789;
pub const VALUE_D:u64 = 91;
pub const ORDER:u64 = 44497;

#[macro_export]
macro_rules! test_one {
    ($name: ident, $a:expr) => {
        self::concat_idents!(test_name=test_one_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let one:$name = $name::one();
                assert_eq!(&a * &one, a);
                assert_eq!(&one * &a, a);
            }
        });
    }
}

#[macro_export]
macro_rules! test_zero {
    ($name: ident, $a:expr) => {
        self::concat_idents!(test_name=test_zero_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let zero:$name = $name::zero();
                assert_eq!(&a + &zero, a);
                assert_eq!(&zero + &a, a);
            }
        });
    }
}


#[macro_export]
macro_rules! test_associativity {
    ($name: ident, $op: ident, $a: expr, $b: expr, $c: expr) => {
        self::concat_idents!(test_name=test_, $op, _associativity_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let b = $b;
                let c = $c;
                let res1 = (&a).$op(&(&b).$op(&c));
                let res2 = (&(&a).$op(&b)).$op(&c);
                assert_eq!(res1, res2);
            }
        });
    }
}

#[macro_export]
macro_rules! test_commutativity {
    ($name: ident, $op: ident, $a: expr, $b: expr) => {
        self::concat_idents!(test_name=test_, $op, _commutativity_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let b = $b;
                let res1 = (&a).$op(&b);
                let res2 = (&b).$op(&a);
                assert_eq!(res1, res2);
            }
        });
    }
}

#[macro_export]
macro_rules! test_double_and_halve {
    ($name: ident, $a: expr) => {
        self::concat_idents!(test_name=test_double_and_halve_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let b = a.double();
                if (! a.is_zero()) {
                    assert_ne!(&a, &b);
                }
                
                let c = b.halve();
                assert_eq!(&a, &c);
            }
        });
    }
}

#[macro_export]
macro_rules! test_distributivity {
    ($name: ident, $add: ident, $mul:ident, $a: expr, $b: expr, $c: expr) => {
        self::concat_idents!(test_name=test_,$add,_and_,$mul,distributivity_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let b = $b;
                let c = $c;
                let res1 = (&a).$mul(&(&b).$add(&c));
                let res2 = (&(&a).$mul(&b)).$add(&(&a).$mul(&c));
                assert_eq!(res1, res2);
            }
        });
    }
}

#[macro_export]
macro_rules! test_square_and_sqrt {
    ($name: ident, $a: expr) => {
        self::concat_idents!(test_name=test_square_and_sqrt_for_, $name {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $a;
                let b = a.square();
                if (! a.is_zero()) {
                    assert_ne!(&a, &b);
                }
                
                // keep in mind that sqrt always has two results (or None)
                match b.sqrt() {
                    Some((s1, s2)) => assert!(a == s1 || a == s2),
                    None           => assert!(false)
                }
            }
        });
    }
}