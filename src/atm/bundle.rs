use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::atm::denomination::Denomination;

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

    pub fn load_bills_for(&mut self, quantity: i32, denomination: i32) {
        let actual = self.bills.get(&denomination).unwrap_or(&0).to_owned();
        self.bills.insert(denomination, quantity + actual);
    }

    pub fn get_total_amount(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_amount_returns_total_for_all_denominations() {
        let bundle = Bundle::new();

        assert_eq!(0, bundle.get_total_amount());
    }
}