use crate::helper_types::{TransactionId, UserId};
use crate::transaction::Transaction;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Default)]
struct Account {
    available: i128,
    held: i128,
    total: i128,
    locked: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DisputeState {
    Initial,
    Disputed,
    Resolved,
    ChargedBack,
}

#[derive(Clone, Debug)]
struct TransactionRecord {
    transaction: Transaction,
    state: DisputeState,
}

impl TransactionRecord {
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            state: DisputeState::Initial,
        }
    }

    pub fn dispute(&mut self) {
        assert_eq!(self.state, DisputeState::Initial);
        self.state = DisputeState::Disputed
    }

    pub fn resolve(&mut self) {
        assert_eq!(self.state, DisputeState::Disputed);
        self.state = DisputeState::Resolved
    }

    pub fn chargeback(&mut self) {
        assert_eq!(self.state, DisputeState::Disputed);
        self.state = DisputeState::ChargedBack
    }
}


#[derive(Clone, Debug, Default)]
pub struct AccountManager {
    accounts: HashMap<UserId, Account>,
    transactions: HashMap<TransactionId, TransactionRecord>,
}
