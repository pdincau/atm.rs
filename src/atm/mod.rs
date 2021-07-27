use std::cmp;

use strum::IntoEnumIterator;
use thiserror::Error;

use crate::atm::bundle::Bundle;
use crate::atm::denomination::Denomination;
use AtmError::NeedsService;

mod bundle;
mod denomination;

#[allow(dead_code)]
pub struct Atm {
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

        let bundle_backup = Bundle::new().load_all_bills_of_bundle(self.bundle.clone());

        for denomination in Denomination::iter() {
            let quantity = self.bundle.get(denomination);
            if quantity > 0 {
                let bills_for_withdrawal = cmp::min(quantity, denomination.bills_for(remainder));
                withdrawal.load_bills(bills_for_withdrawal, denomination);
                remainder -= denomination.times(bills_for_withdrawal);
                self.bundle.unload_bills(bills_for_withdrawal, denomination);
            }
        }

        if remainder != 0 {
            self.bundle = bundle_backup;
            Err(NeedsService(remainder.to_string()))
        } else {
            Ok(withdrawal)
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum AtmError {
    #[error("Not enough money (remainder is {:?})", .0)]
    NeedsService(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use AtmError::NeedsService;

    #[test]
    fn withdraw_fails_if_there_is_not_enough_money() {
        let mut atm = Atm::new();

        assert_eq!(NeedsService(25.to_string()), atm.withdraw(25).unwrap_err());
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

        atm.bundle.load_all_bills([0, 0, 4, 2]);

        let bundle = atm.withdraw(50).unwrap();

        assert_eq!(50, bundle.get_total_amount());
        assert_eq!(Bundle::new(), atm.bundle);
    }

    #[test]
    fn withdraw_uses_bigger_denominations_first() {
        let mut atm = Atm::new();

        atm.bundle.load_all_bills([0, 10, 10, 10]);

        assert_eq!(
            Bundle::new().load_all_bills([0, 2, 1, 0]),
            atm.withdraw(50).unwrap()
        );
        assert_eq!(Bundle::new().load_all_bills([0, 8, 9, 10]), atm.bundle);
    }

    #[test]
    fn withdraw_fails_for_odd_amounts_and_puts_money_back() {
        let mut atm = Atm::new();

        atm.bundle.load_all_bills([10, 10, 10, 10]);

        check_remainder_and_reload_money(&mut atm, 11, 1);
        check_remainder_and_reload_money(&mut atm, 27, 2);
        check_remainder_and_reload_money(&mut atm, 389, 4);
        check_remainder_and_reload_money(&mut atm, 889, 39);
        check_remainder_and_reload_money(&mut atm, 10850, 10000);
    }

    fn check_remainder_and_reload_money(atm: &mut Atm, amount: i32, reminder: i32) {
        assert_eq!(
            NeedsService(reminder.to_string()),
            atm.withdraw(amount).unwrap_err()
        );
        assert_eq!(Bundle::new().load_all_bills([10, 10, 10, 10]), atm.bundle);
    }
}
