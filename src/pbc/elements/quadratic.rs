use gmp::mpz::Mpz;
use std::ops;
use num_traits::{One, Zero};
use std::rc::Rc;
use super::QuadraticField;
use super::traits::{Field, Element};

pub struct Quadratic<E: Element, F: Field<E>> {
    x: E,
    y: E,
    field: Rc<QuadraticField<E, F>>,
}

impl<E, F>  Quadratic<E,F>
    where   E: Element,
            F: Field<E> {
    pub fn new(x: E, y: E, field: Rc<QuadraticField<E, F>>) -> Quadratic<E, F> {
        Quadratic {
            x,
            y,
            field
        }
    }
}
