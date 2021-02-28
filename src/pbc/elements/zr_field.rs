use gmp::mpz::Mpz;
use super::Zr;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct ZrField {
    order: Rc<Mpz>,
    zero: Zr,
    one: Zr,
}

impl ZrField {
    pub fn new(order: Mpz) -> ZrField {
        let field = ZrField {
            order: Rc::new(order),
            zero: Zr{value: Mpz::from(0), field: None},
            one: Zr{value: Mpz::from(1), field: None},
        };
        field
    }

    pub fn order(&self) -> Rc<Mpz> {
        self.order.clone()
    }
}