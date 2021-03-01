use gmp::mpz::Mpz;
use std::ops;
use num_traits::{One, Zero};
use std::rc::Rc;
use super::QuadraticField;
use super::traits::{Field, Element};
/*
pub struct Quadratic<E: Element, F: Field> {
    x: E,
    y: E,
    field: Rc<F>,
}

impl Quadratic {
    pub fn new(x: Mpz, y: Mpz, field: Rc<QuadraticField>) {
        Quadratic {
            x,
            y,
            field
        }
    }
}
*/