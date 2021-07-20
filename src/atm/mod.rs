use std::collections::HashMap;

use thiserror::Error;

struct Atm {
    bills: HashMap<i32, i32>
}

impl Atm {
    pub fn new() -> Atm {
        Atm {
            bills: HashMap::new()
        }
    }

    pub fn bills_for(&self, denomination: i32) -> i32 {
        self.bills.get(&denomination).unwrap_or(&0).to_owned()
    }

    pub fn load_bills_for(&mut self, quantity: i32, denomination: i32) {
        let actual = self.bills.get(&denomination).unwrap_or(&0).to_owned();
        self.bills.insert(denomination, quantity + actual);
    }

    pub fn withdraw(&self, _amount: i32) -> Result<(), AtmError>{
        Err(AtmError::NeedsService)
    }
}

#[derive(Debug, Error)]
pub enum AtmError {
    #[error("This ATM requires servicing")]
    NeedsService
}

#[cfg(test)]
mod tests {
    use claim::assert_err;

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

        assert_err!(atm.withdraw(25));
    }
}