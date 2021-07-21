use std::collections::HashMap;

use thiserror::Error;

struct Atm {
    bills: HashMap<i32, i32>,
}

impl Atm {
    pub fn new() -> Atm {
        let mut initial_bills = HashMap::new();
        initial_bills.insert(5, 0);
        initial_bills.insert(10, 0);
        initial_bills.insert(20, 0);
        initial_bills.insert(50, 0);

        Atm {
            bills: initial_bills
        }
    }

    pub fn bills_for(&self, denomination: i32) -> i32 {
        self.bills.get(&denomination).unwrap_or(&0).to_owned()
    }

    pub fn load_bills_for(&mut self, quantity: i32, denomination: i32) {
        let actual = self.bills.get(&denomination).unwrap_or(&0).to_owned();
        self.bills.insert(denomination, quantity + actual);
    }

    pub fn withdraw(&self, amount: i32) -> Result<HashMap<i32, i32>, AtmError> {
        let mut bundle = HashMap::new();
        let mut remainder = amount;

        for (denomination, quantity) in self.bills.clone() {
            if remainder > denomination && quantity > 0 && remainder / denomination < quantity {
                bundle.insert(denomination, remainder / denomination);
                remainder -= denomination * quantity;
            }
        }

        if remainder > 0 {
            Err(AtmError::NeedsService)
        } else {
            Ok(bundle)
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum AtmError {
    #[error("This ATM requires servicing")]
    NeedsService,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bills_for_sets_current_number_of_bills_for_denomination() {
        let mut atm = Atm::new();

        assert_eq!(0, atm.bills_for(5));
        assert_eq!(0, atm.bills_for(10));

        atm.load_bills_for(3, 5);

        assert_eq!(3, atm.bills_for(5));
        assert_eq!(0, atm.bills_for(10));
    }

    #[test]
    fn load_bills_for_updated_current_number_of_bills_if_some_are_already_present() {
        let mut atm = Atm::new();

        atm.load_bills_for(1, 20);

        assert_eq!(1, atm.bills_for(20));

        atm.load_bills_for(3, 20);

        assert_eq!(4, atm.bills_for(20));
    }

    #[test]
    fn withdraw_fails_if_there_is_not_enough_money() {
        let atm = Atm::new();

        assert_eq!(AtmError::NeedsService, atm.withdraw(25).unwrap_err());
    }

    #[test]
    fn withdraw_returns_bundle_for_desired_amount() {
        let mut atm = Atm::new();

        atm.load_bills_for(10, 5);

        let bundle = atm.withdraw(25).unwrap();

        let mut amount = 0;

        for (denomination, quantity) in bundle {
            amount += denomination * quantity;
        }

        assert_eq!(amount, 25);
    }
}
