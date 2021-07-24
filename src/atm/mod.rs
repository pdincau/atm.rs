use std::cmp;

use strum::IntoEnumIterator;
use thiserror::Error;

use crate::atm::bundle::Bundle;
use crate::atm::denomination::Denomination;

mod bundle;
mod denomination;

#[allow(dead_code)]
struct Atm {
    pub bundle: Bundle,
}

impl Atm {
    #[allow(dead_code)]
    pub fn new() -> Atm {
        Atm {
            bundle: Bundle::new(),
        }
    }

    #[allow(dead_code)]
    pub fn withdraw(&mut self, amount: i32) -> Result<Bundle, AtmError> {
        let mut withdrawal = Bundle::new();
        let mut remainder = amount;

        for denomination in Denomination::iter() {
            let quantity = self.bundle.get(denomination);
            if quantity > 0 {
                let bills_for_remainder = cmp::min(quantity, denomination.bills_for(remainder));
                withdrawal.load_bills(bills_for_remainder, denomination);
                remainder -= denomination.times(bills_for_remainder);
                self.bundle.unload_bills(bills_for_remainder, denomination);
            }
        }

        if remainder != 0 {
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
        let mut atm = Atm::new();

        assert_eq!(AtmError::NeedsService, atm.withdraw(25).unwrap_err());
    }

    #[test]
    fn withdraw_exact_amount_for_one_denomination() {
        let mut atm = Atm::new();

        atm.bundle.load_bills(11, Denomination::Five);

        let bundle = atm.withdraw(25).unwrap();

        assert_eq!(25, bundle.get_total_amount());
        assert_eq!(6, atm.bundle.get(Denomination::Five));
    }

    #[test]
    fn withdraw_exact_amount_with_two_denominations() {
        let mut atm = Atm::new();

        atm.bundle.load_bills(2, Denomination::Five);
        atm.bundle.load_bills(4, Denomination::Ten);

        let bundle = atm.withdraw(50).unwrap();

        assert_eq!(50, bundle.get_total_amount());
        assert_eq!(0, atm.bundle.get(Denomination::Five));
        assert_eq!(0, atm.bundle.get(Denomination::Ten));
    }
}
