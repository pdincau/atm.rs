use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum Denomination {
    Fifty = 50,
    Twenty = 20,
    Ten = 10,
    Five = 5,
}


impl Denomination {

    pub fn at(index: usize) -> Denomination {
        match index {
            0 => Denomination::Fifty,
            1 => Denomination::Twenty,
            2 => Denomination::Ten,
            _ => Denomination::Five,
        }
    }

    pub fn times(&self, quantity: i32) -> i32 {
        *self as i32 * quantity
    }

    pub fn bills_for(&self, remainder: i32) -> i32 {
        remainder / *self as i32
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

    #[test]
    fn bills_for_returns_number_of_bills_whose_sum_is_not_greater_than_quantity() {
        assert_eq!(1, Denomination::Five.bills_for(5));
        assert_eq!(1, Denomination::Five.bills_for(7));
        assert_eq!(2, Denomination::Five.bills_for(11));
    }
}
