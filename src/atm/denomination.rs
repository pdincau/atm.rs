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

    pub fn times(&self, quantity: i32) -> i32 {
        self.value() * quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_returns_total_based_on_denomination() {
        assert_eq!(20, Denomination::Ten.times(2));
        assert_eq!(150, Denomination::Fifty.times(3));
    }
}
