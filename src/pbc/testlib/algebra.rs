pub use concat_idents::concat_idents;

pub const ORDER:u64 = 44497;

#[macro_export]
macro_rules! test_one {
    ($elem_type: ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_one_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
                let one:$elem_type = $elem_type::one();
                assert_eq!(&a * &one, a);
                assert_eq!(&one * &a, a);
            }
        });
    }
}

#[macro_export]
macro_rules! test_zero {
    ($elem_type: ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_zero_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
                let zero:$elem_type = $elem_type::zero();
                assert_eq!(&a + &zero, a);
                assert_eq!(&zero + &a, a);
            }
        });
    }
}


#[macro_export]
macro_rules! test_associativity {
    ($elem_type: ident, $op: ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_, $op, _associativity_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
                let b = $field_type::random($field);
                let c = $field_type::random($field);
                let res1 = (&a).$op(&(&b).$op(&c));
                let res2 = (&(&a).$op(&b)).$op(&c);
                assert_eq!(res1, res2);
            }
        });
    }
}

#[macro_export]
macro_rules! test_commutativity {
    ($elem_type: ident, $op: ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_, $op, _commutativity_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
                let b = $field_type::random($field);
                let res1 = (&a).$op(&b);
                let res2 = (&b).$op(&a);
                assert_eq!(res1, res2);
            }
        });
    }
}

#[macro_export]
macro_rules! test_double_and_halve {
    ($elem_type: ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_double_and_halve_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
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
    ($elem_type: ident, $add: ident, $mul:ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_,$add,_and_,$mul,distributivity_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
                let b = $field_type::random($field);
                let c = $field_type::random($field);
                let res1 = (&a).$mul(&(&b).$add(&c));
                let res2 = (&(&a).$mul(&b)).$add(&(&a).$mul(&c));
                assert_eq!(res1, res2);
            }
        });
    }
}

#[macro_export]
macro_rules! test_square_and_sqrt {
    ($elem_type: ident, $field_type: ident, $field:expr) => {
        self::concat_idents!(test_name=test_square_and_sqrt_for_, $elem_type {
            #[allow(non_snake_case)]
            #[test]
            fn test_name() {
                let a = $field_type::random($field);
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