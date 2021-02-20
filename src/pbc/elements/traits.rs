pub trait Square {
    fn square(&self) -> Self;
}

pub trait Double {
    fn double(&self) -> Self;
}
pub trait Halve {
    fn halve(&self) -> Self;
}

pub trait SquareRoot {
    type Item;
    fn sqrt(&self) -> Option<(Self::Item, Self::Item)>;
}