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

#[derive(Clone, Debug, Default)]
pub struct AccountManager {
    accounts: HashMap<UserId, Account>,
    transactions: HashMap<TransactionId, TransactionRecord>,
}
