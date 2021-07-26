use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::atm::denomination::Denomination;

#[derive(Debug, PartialEq)]
pub struct Bundle {
    bills: HashMap<Denomination, i32>,
}

impl Bundle {
    pub fn new() -> Bundle {
        let mut initial_bills = HashMap::new();
        for denomination in Denomination::iter() {
            initial_bills.insert(denomination, 0);
        }

        Bundle {
            bills: initial_bills,
        }
    }

    pub fn get(&self, denomination: Denomination) -> i32 {
        self.bills.get(&denomination).unwrap_or(&0).to_owned()
    }

    pub fn load_bills(&mut self, quantity: i32, denomination: Denomination) {
        let actual = self.bills.get(&denomination).unwrap_or(&0).to_owned();
        self.bills.insert(denomination, quantity + actual);
    }

    pub fn unload_bills(&mut self, quantity: i32, denomination: Denomination) {
        self.load_bills(-quantity, denomination);
    }

    #[allow(dead_code)]
    pub fn load_all_bills(&mut self, quantities: [i32; 4]) {
        for (index, quantity) in quantities.iter().enumerate() {
            let denomination = match index {
                0 => Denomination::Fifty,
                1 => Denomination::Twenty,
                2 => Denomination::Ten,
                _ => Denomination::Five,
            };
            self.load_bills(*quantity, denomination);
        }
    }

    #[allow(dead_code)]
    pub fn get_total_amount(&self) -> i32 {
        let mut amount = 0;
        for denomination in Denomination::iter() {
            amount += denomination.times(*self.bills.get(&denomination).unwrap_or(&0));
        }
        amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bills_sets_current_number_of_bills_for_denomination() {
        let mut bundle = Bundle::new();

        assert_eq!(0, bundle.get(Denomination::Five));
        assert_eq!(0, bundle.get(Denomination::Ten));

        bundle.load_bills(3, Denomination::Five);

        assert_eq!(3, bundle.get(Denomination::Five));
        assert_eq!(0, bundle.get(Denomination::Ten));
    }

    #[test]
    fn load_bills_updates_current_number_of_bills_if_some_are_already_present() {
        let mut bundle = Bundle::new();

        bundle.load_bills(1, Denomination::Twenty);

        assert_eq!(1, bundle.get(Denomination::Twenty));

        bundle.load_bills(3, Denomination::Twenty);

        assert_eq!(4, bundle.get(Denomination::Twenty));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn total_amount_returns_total_for_all_denominations() {
        let mut bundle = Bundle::new();

        assert_eq!(0, bundle.get_total_amount());

        bundle.load_all_bills([1, 2, 0, 10]);

        assert_eq!(10 * 5 + 2 * 20 + 1 * 50, bundle.get_total_amount());
    }

    #[test]
    fn equals_is_based_on_contained_bills() {
        let mut bundle = Bundle::new();

        let mut other = Bundle::new();

        assert_eq!(other, bundle);

        other.load_bills(2, Denomination::Five);
        other.load_bills(1, Denomination::Twenty);

        assert_ne!(Bundle::new(), other);

        bundle.load_bills(2, Denomination::Five);
        bundle.load_bills(1, Denomination::Twenty);

        assert_eq!(bundle, other);
    }

    #[test]
    fn load_all_bills_sets_all_denominations() {
        let mut bundle = Bundle::new();

        bundle.load_all_bills([1, 2, 3, 4]);

        assert_eq!(1, bundle.get(Denomination::Fifty));
        assert_eq!(2, bundle.get(Denomination::Twenty));
        assert_eq!(3, bundle.get(Denomination::Ten));
        assert_eq!(4, bundle.get(Denomination::Five));
    }
}
