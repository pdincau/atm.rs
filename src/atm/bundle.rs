use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::atm::denomination::Denomination;

#[derive(Debug)]
pub struct Bundle {
    bills: HashMap<i32, i32>,
}

impl Bundle {
    pub fn new() -> Bundle {
        let mut initial_bills = HashMap::new();
        for denomination in Denomination::iter() {
            initial_bills.insert(denomination.value(), 0);
        }

        Bundle {
            bills: initial_bills
        }
    }

    pub fn get(&self, denomination: i32) -> i32 {
        self.bills.get(&denomination).unwrap_or(&0).to_owned()
    }

    pub fn load_bills(&mut self, quantity: i32, denomination: i32) {
        let actual = self.bills.get(&denomination).unwrap_or(&0).to_owned();
        self.bills.insert(denomination, quantity + actual);
    }

    pub fn get_total_amount(&self) -> i32 {
        let mut amount = 0;
        for denomination in Denomination::iter() {
            amount += denomination.value() * self.bills.get(&denomination.value()).unwrap_or(&0);
        }
        amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bills_for_sets_current_number_of_bills_for_denomination() {
        let mut bundle = Bundle::new();

        assert_eq!(0, bundle.get(5));
        assert_eq!(0, bundle.get(10));

        bundle.load_bills(3, 5);

        assert_eq!(3, bundle.get(5));
        assert_eq!(0, bundle.get(10));
    }

    #[test]
    fn load_bills_for_updated_current_number_of_bills_if_some_are_already_present() {
        let mut bundle = Bundle::new();

        bundle.load_bills(1, 20);

        assert_eq!(1, bundle.get(20));

        bundle.load_bills(3, 20);

        assert_eq!(4, bundle.get(20));
    }

    #[test]
    fn total_amount_returns_total_for_all_denominations() {
        let mut bundle = Bundle::new();

        assert_eq!(0, bundle.get_total_amount());

        bundle.load_bills(10, 5);
        bundle.load_bills(2, 20);
        bundle.load_bills(1, 50);

        assert_eq!(10*5+2*20+1*50, bundle.get_total_amount());
    }
}