use strum::IntoEnumIterator;
use thiserror::Error;

use crate::atm::bundle::Bundle;
use crate::atm::denomination::Denomination;

mod bundle;
mod denomination;

struct Atm {
    bundle: Bundle,
}

impl Atm {
    pub fn new() -> Atm {
        Atm {
            bundle: Bundle::new(),
        }
    }

    pub fn bills_for(&self, denomination: i32) -> i32 {
        self.bundle.get(denomination).to_owned()
    }

    pub fn load_bills_for(&mut self, quantity: i32, denomination: i32) {
        self.bundle.load_bills_for(quantity, denomination);
    }

    pub fn withdraw(&self, amount: i32) -> Result<Bundle, AtmError> {
        let mut withdrawal = Bundle::new();
        let mut remainder = amount;

        for denomination in Denomination::iter() {
            let quantity = self.bundle.get(denomination.value());
            if remainder > denomination.value() && quantity > 0 && remainder / denomination.value() < quantity {
                withdrawal.load_bills_for(remainder / denomination.value(), denomination.value());
                remainder -= denomination.value() * quantity;
            }
        }

        if remainder > 0 {
            Err(AtmError::NeedsService)
        } else {
            Ok(withdrawal)
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

        assert_eq!(25, bundle.get_total_amount());
    }
}
