use crate::helper_types::{TransactionId, UserId};
use crate::transaction::{Amount, Transaction};
use std::collections::HashMap;
use Transaction::{Chargeback, Deposit, Dispute, Resolve, Withdrawal};

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

impl AccountManager {
    pub fn handle_transaction(&mut self, transaction: Transaction) -> Option<()> {
        match transaction {
            Deposit { user, tx, amount } => self.handle_deposit(user, tx, amount),
            Withdrawal {
                user,
                tx: _,
                amount,
            } => self.handle_withdrawal(user, amount),
            Dispute { user, tx } => self.handle_dispute(user, tx),
            Resolve { user, tx } => self.handle_resolve(user, tx),
            Chargeback { user, tx } => self.handle_chargeback(user, tx),
        };
        Some(())
    }

    fn handle_deposit(&mut self, user: UserId, tx: TransactionId, amount: Amount) {
        //I guess only deposits should create accounts
        let account = self
            .accounts
            .entry(user)
            .or_insert_with(|| Account::default());
        account.available += amount.0 as i128;
        account.total += amount.0 as i128;
        self.transactions
            .insert(tx, TransactionRecord::new(Deposit { user, tx, amount }));
    }

    fn handle_withdrawal(&mut self, user: UserId, amount: Amount) {
        if let Some(account @ Account { locked: false, .. }) = self.accounts.get_mut(&user) {
            if account.available >= amount.0 as i128 {
                account.available -= amount.0 as i128;
                account.total -= amount.0 as i128;
            }
        }
    }

    fn handle_dispute(&mut self, user: UserId, tx: TransactionId) {
        use DisputeState::Initial;
        if let Some(account @ Account { locked: false, .. }) = self.accounts.get_mut(&user) {
            self.transactions.entry(tx).and_modify(|entry| {
                if let TransactionRecord {
                    transaction: Deposit { amount, .. },
                    state: Initial,
                } = entry
                {
                    account.available -= amount.0 as i128;
                    account.held += amount.0 as i128;
                    entry.dispute();
                }
            });
        }
    }

    fn handle_resolve(&mut self, user: UserId, tx: TransactionId) {
        use DisputeState::Disputed;
        if let Some(account @ Account { locked: false, .. }) = self.accounts.get_mut(&user) {
            self.transactions.entry(tx).and_modify(|entry| {
                if let TransactionRecord {
                    transaction: Deposit { amount, .. },
                    state: Disputed,
                } = entry
                {
                    account.available -= amount.0 as i128;
                    account.held += amount.0 as i128;
                    entry.resolve();
                }
            });
        }
    }

    fn handle_chargeback(&mut self, user: UserId, tx: TransactionId) {
        use DisputeState::Disputed;
        if let Some(account @ Account { locked: false, .. }) = self.accounts.get_mut(&user) {
            self.transactions.entry(tx).and_modify(|entry| {
                if let TransactionRecord {
                    transaction: Deposit { amount, .. },
                    state: Disputed,
                } = entry
                {
                    account.held -= amount.0 as i128;
                    account.total -= amount.0 as i128;
                    account.locked = true;
                    entry.chargeback();
                }
            });
        }
    }
}
