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

    pub fn bills_for(&self, denomination: Denomination) -> i32 {
        self.bundle.get(denomination).to_owned()
    }

    pub fn load_bills_for(&mut self, quantity: i32, denomination: Denomination) {
        self.bundle.load_bills(quantity, denomination);
    }

    pub fn withdraw(&self, amount: i32) -> Result<Bundle, AtmError> {
        let mut withdrawal = Bundle::new();
        let mut remainder = amount;

        for denomination in Denomination::iter() {
            let quantity = self.bundle.get(denomination);
            if remainder > denomination.value()
                && quantity > 0
                && remainder / denomination.value() < quantity
            {
                withdrawal.load_bills(remainder / denomination.value(), denomination);
                remainder -= denomination.times(quantity);
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
    fn withdraw_fails_if_there_is_not_enough_money() {
        let atm = Atm::new();

        assert_eq!(AtmError::NeedsService, atm.withdraw(25).unwrap_err());
    }

    #[test]
    fn withdraw_returns_bundle_for_desired_amount() {
        let mut atm = Atm::new();

        atm.load_bills_for(10, Denomination::Five);

        let bundle = atm.withdraw(25).unwrap();

        assert_eq!(25, bundle.get_total_amount());
    }
}
