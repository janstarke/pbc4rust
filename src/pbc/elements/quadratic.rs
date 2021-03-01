use gmp::mpz::Mpz;
use std::ops;
use num_traits::{One, Zero};
use std::rc::Rc;
use super::QuadraticField;

pub struct Quadratic {
    x: Mpz,
    y: Mpz,
    field: Option<Rc<QuadraticField>>,
}