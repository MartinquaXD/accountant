use crate::helper_types::{TransactionId, UserId, DECIMAL_PRECISION, FLOAT_BASE};
use csv::{StringRecord, StringRecordIter};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Amount(pub u64);

impl FromStr for Amount {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decimal_index = s.find('.').ok_or_else(|| "expected decimal")?;
        let decimal_slice = &s[decimal_index + 1..];
        if decimal_slice.len() > DECIMAL_PRECISION {
            return Err(format!("at most {} decimal digits allowed", DECIMAL_PRECISION).into());
        }
        let missing_zeros = DECIMAL_PRECISION - decimal_slice.len();
        let decimal = decimal_slice.parse::<u64>()?;
        let decimal = decimal
            .checked_mul(10u64.pow(missing_zeros as u32))
            .unwrap();
        let whole_number = s[..decimal_index]
            .parse::<u64>()?
            .checked_mul(FLOAT_BASE as u64)
            .ok_or_else(|| "Overflow")?;
        Ok(Self(
            whole_number
                .checked_add(decimal)
                .ok_or_else(|| "Overflow")?,
        ))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Transaction {
    Deposit {
        user: UserId,
        tx: TransactionId,
        amount: Amount,
    },
    Withdrawal {
        user: UserId,
        tx: TransactionId,
        amount: Amount,
    },
    Dispute {
        user: UserId,
        tx: TransactionId,
    },
    Resolve {
        user: UserId,
        tx: TransactionId,
    },
    Chargeback {
        user: UserId,
        tx: TransactionId,
    },
}

impl Transaction {
    fn try_create_deposit_transaction(mut it: StringRecordIter) -> Option<Self> {
        Some(Self::Deposit {
            user: it.next()?.parse().ok()?,
            tx: it.next()?.parse().ok()?,
            amount: it.next()?.parse().ok()?,
        })
    }

    fn try_create_withdrawal_transaction(mut it: StringRecordIter) -> Option<Self> {
        Some(Self::Withdrawal {
            user: it.next()?.parse().ok()?,
            tx: it.next()?.parse().ok()?,
            amount: it.next()?.parse().ok()?,
        })
    }

    fn try_create_dispute_transaction(mut it: StringRecordIter) -> Option<Self> {
        Some(Self::Dispute {
            user: it.next()?.parse().ok()?,
            tx: it.next()?.parse().ok()?,
        })
    }

    fn try_create_resolve_transaction(mut it: StringRecordIter) -> Option<Self> {
        Some(Self::Resolve {
            user: it.next()?.parse().ok()?,
            tx: it.next()?.parse().ok()?,
        })
    }

    fn try_create_chargeback_transaction(mut it: StringRecordIter) -> Option<Self> {
        Some(Self::Chargeback {
            user: it.next()?.parse().ok()?,
            tx: it.next()?.parse().ok()?,
        })
    }
}

impl TryFrom<StringRecord> for Transaction {
    type Error = &'static str;
    fn try_from(mut row: StringRecord) -> Result<Self, Self::Error> {
        row.trim();
        let mut it = row.iter();
        match it.next() {
            Some("deposit") => Self::try_create_deposit_transaction(it)
                .ok_or("wrong format for deposit transaction"),
            Some("withdrawal") => Self::try_create_withdrawal_transaction(it)
                .ok_or("wrong format for withdrawal transaction"),
            Some("dispute") => Self::try_create_dispute_transaction(it)
                .ok_or("wrong format for dispute transaction"),
            Some("resolve") => Self::try_create_resolve_transaction(it)
                .ok_or("wrong format for resolve transaction"),
            Some("chargeback") => Self::try_create_chargeback_transaction(it)
                .ok_or("wrong format for chargeback transaction"),
            _ => Err("unknown transaction type"),
        }
    }
}

#[test]
fn test_convert_amount() {
    assert_eq!(Amount(123456789), "12345.6789".parse::<Amount>().unwrap());
    assert_eq!(Amount(123450789), "12345.0789".parse::<Amount>().unwrap());
    assert_eq!(Amount(123455000), "12345.5".parse::<Amount>().unwrap());
    assert_eq!(Amount(1), "0.0001".parse::<Amount>().unwrap());
    //Too many decimal digits
    assert!("12345.07891".parse::<Amount>().is_err());
    //2 decimals
    assert!("12345.07.0".parse::<Amount>().is_err());
    //non digits included
    assert!("asd12312.12".parse::<Amount>().is_err());
    //used comma instead of point for decimal
    assert!("123123,12".parse::<Amount>().is_err());
}

#[test]
fn test_create_deposit() {
    assert_eq!(
        Transaction::try_from(StringRecord::from(vec!["deposit", "12", "1", "123.0"])),
        Ok(Transaction::Deposit {
            user: 12,
            tx: 1,
            amount: Amount(1230000)
        })
    );
    //amount has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["deposit", "12", "1", "123"])).is_err());
    //user id has wrong format
    assert!(
        Transaction::try_from(StringRecord::from(vec!["deposit", "-1", "1", "123.0"])).is_err()
    );
    //user id has wrong format
    assert!(
        Transaction::try_from(StringRecord::from(vec!["deposit", "1", "-1", "123.0"])).is_err()
    );
    //some item is missing
    assert!(Transaction::try_from(StringRecord::from(vec!["deposit", "12", "1"])).is_err());
}

#[test]
fn test_create_withdrawal() {
    assert_eq!(
        Transaction::try_from(StringRecord::from(vec!["withdrawal", "12", "1", "123.0"])),
        Ok(Transaction::Withdrawal {
            user: 12,
            tx: 1,
            amount: Amount(1230000)
        })
    );
    //amount has wrong format
    assert!(
        Transaction::try_from(StringRecord::from(vec!["withdrawal", "12", "1", "123"])).is_err()
    );
    //user id has wrong format
    assert!(
        Transaction::try_from(StringRecord::from(vec!["withdrawal", "-1", "1", "123.0"])).is_err()
    );
    //user id has wrong format
    assert!(
        Transaction::try_from(StringRecord::from(vec!["withdrawal", "1", "-1", "123.0"])).is_err()
    );
    //some item is missing
    assert!(Transaction::try_from(StringRecord::from(vec!["withdrawal", "12", "1"])).is_err());
}

#[test]
fn test_create_dispute() {
    assert_eq!(
        Transaction::try_from(StringRecord::from(vec!["dispute", "12", "1"])),
        Ok(Transaction::Dispute { user: 12, tx: 1 })
    );
    //user id has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["dispute", "-1", "1"])).is_err());
    //user id has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["dispute", "1", "-1"])).is_err());
    //some item is missing
    assert!(Transaction::try_from(StringRecord::from(vec!["dispute", "12"])).is_err());
}

#[test]
fn test_create_resolve() {
    assert_eq!(
        Transaction::try_from(StringRecord::from(vec!["resolve", "12", "1"])),
        Ok(Transaction::Resolve { user: 12, tx: 1 })
    );
    //user id has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["resolve", "-1", "1"])).is_err());
    //user id has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["resolve", "1", "-1"])).is_err());
    //some item is missing
    assert!(Transaction::try_from(StringRecord::from(vec!["resolve", "12"])).is_err());
}

#[test]
fn test_create_chargeback() {
    assert_eq!(
        Transaction::try_from(StringRecord::from(vec!["chargeback", "12", "1"])),
        Ok(Transaction::Chargeback { user: 12, tx: 1 })
    );
    //user id has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["chargeback", "-1", "1"])).is_err());
    //user id has wrong format
    assert!(Transaction::try_from(StringRecord::from(vec!["chargeback", "1", "-1"])).is_err());
    //some item is missing
    assert!(Transaction::try_from(StringRecord::from(vec!["chargeback", "12"])).is_err());
}
