use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum Denomination {
    Five,
    Ten,
    Twenty,
    Fifty,
}

impl Denomination {
    pub fn value(&self) -> i32 {
        match *self {
            Denomination::Five => 5,
            Denomination::Ten => 10,
            Denomination::Twenty => 20,
            Denomination::Fifty => 50,
        }
    }
}