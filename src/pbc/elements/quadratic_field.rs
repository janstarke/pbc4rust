use super::traits::*;
use super::Quadratic;
use std::marker::PhantomData;
use std::rc::Rc;


#[derive(Debug, Clone, PartialEq)]
pub struct QuadraticField<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    //order: Mpz,
    target_field: Rc<F>,
    phantom: PhantomData<E>,
}

impl<E, F> QuadraticField<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    pub fn new(target_field: Rc<F>) -> QuadraticField<E, F> {
        //let order = target_field.order() * target_field.order();
        let field = QuadraticField {
            //order,
            target_field,
            phantom: PhantomData,
        };
        field
    }
}

impl<E, F> HasZero<Quadratic<E, F>> for QuadraticField<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn zero_element(self: Rc<Self>) -> Quadratic<E, F> {
        Quadratic::new(
            Rc::clone(&self.target_field).zero_element(),
            Rc::clone(&self.target_field).zero_element(),
            Rc::clone(&self),
        )
    }
}

impl<E, F> HasOne<Quadratic<E, F>> for QuadraticField<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn one_element(self: Rc<Self>) -> Quadratic<E, F> {
        Quadratic::new(
            Rc::clone(&self.target_field).one_element(),
            Rc::clone(&self.target_field).zero_element(),
            Rc::clone(&self),
        )
    }
}

impl<E, F> Field<Quadratic<E, F>, ComplexElement> for QuadraticField<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn random_element(self: Rc<Self>) -> Quadratic<E, F> {
        Quadratic::new(
            Rc::clone(&self.target_field).random_element(),
            Rc::clone(&self.target_field).random_element(),
            Rc::clone(&self),
        )
    }
}

impl<E, F> FieldOver<Quadratic<E, F>, QuadraticField<E, F>, E, F, AtomicElement> for QuadraticField<E, F>
where
    E: Element<AtomicElement>,
    F: HasZero<E> + HasNqr<E, AtomicElement>,
{
    fn target_field(&self) -> Rc<F> {
        Rc::clone(&self.target_field)
    }
}