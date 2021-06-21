# Simple Payment Engine

I will use this README to discuss some design decisions, confusion about the requirements and what assumptions this implementation makes.

## Assumptions

### Deposits Create Accounts
There is no dedicated operation which creates an user account. With only deposit, withdrawal, dispute, resolve and chargeback it made the most sense to me that a new account can only be created by the deposit operation. Some banks might gift a new user some intial ammount of money in a new account but that would only cause unneccessary confusion here. Every transaction besides deposits will be ignored for a user until that user deposits some money which causes an account to be created.

### Only Positive Transaction Amounts
Another assumption I made was that transactions will never have negative values as amounts. This makes sense since there are 2 separate transaction for increasing and decreasing the balance of an account.

### Locked Accounts Ignore All Associated Transactions
An account gets locked after a successful chargeback. Every following transaction associated with this account will be ignored until the account gets unlocked again. It is currently unclear when that is supposed to happen.

### Only Withdrawals Can Be Disputed
The wording of the exercise seemed to suggest that only withdrawals can be disputed. I guess that also makes sense intuitively.

## Implementation Details

### Using u64 to Store Amounts
CSV files used to feed the process represent amounts with floating point numbers with 4 decimal digits. As far as I know floating point numbers should never be used to store something as a balance because they can introduce rounding errors. I therefore decided to convert floating points amounts to unisgned integers. Because an amount can have at most 4 decimal digits I know that the smallest amount will be 0.0001. This will be 1 unit.

### Not Using Serde
I'm not that familiar with implenting specialized deserializitaion requirements with serde. It could be that there is an easy way to deserialize the amounts in the afore mentioned way, but I'm not sure. Because parsing the transactions seemed quite straight forward and it's rather easy to parse data with the Rust standard library, I decided to parse the CSV rows myself. For a more serious project I would probably bite the bullet and implement custom deserialization logic with serde.

### Using i128 to Store Account Balance
As far as I can tell it should never be possible for an account to have a negative balance. Deposits will always increase the balance of an account and withdrawals will only succeed if there is anough money in the account. Although it is currently not strictly necessary to be able to support negative balances in accounts, I decided to future prrof the implementation a little bit in that regard by using i128 to store the account balance.

### Implementing The Dispute State Machine
While implementing the dispute mechanism I was a little incertain how to interpret the requirements. Originally I thought the process was withdrawal -> dispute -> resolve -> chargeback. After some back and fourth I came to the conclusion that a dispute is settled either by a resolve or a charge back. (withdrawal -> dispute -> resolve or chargeback).
I would have loved to implement that state machine entirely with the Rust type system but stumpled about the problem that I needed to store multiple such states in a collection which requires using an enum (AFAIK). To make it as hard as possible for the developer to make errors in the dispute process I decided to create functions like "dispute()" or "resolve()" to not make the developer instantiate an enum variant direclty. Those functions assert that the transaction is in the correct state before trnsitioning to the next state.

### Low Test Coverage
Ideally all the code should be covered carefully but since I had to bring the project to conclusion in a somewhat timely manner I decided to skip some of the tests. I put most effort into testing the parsing of amounts and CSV rows in general. Those are th parts which take unsanitized user input and prepare them for use in the program. After the data has been put into Rust structs, the type system gave me confidence in the correctness of the code.

### Error Handling
Similar to the test coverage I didn't put too much effort into creating specialized and meaningful errors. This would of course be different for a commercial application. There are also some places where I would able to determine errors but decided to just ignore them because it was not clear how to handle all sorts of errors.
For example what happens if a user tries to dispute a withdrawal which is not even associated to his/her account? Is it an error if a transaction tries to modify a locked account? Is it necessary to guard against malformed CSV rows? ...

### Not Using Multi Threading
I would have loved to use something like tokio or hand crafted multi threading to speed up the pocess but it seems this scenario heavliy relies on the chronological order of execution. Imagine swapping the order of withdrawals and deposits on the same account. In that scenario a withdrawal might fail because the system tried to execute before enought deposits happened to move enough money into the account. One might go through all the transactions first and group them by the account and work on many groups in parallel but it this scenario, where each transaction only causes a very small amount of work, that seems like more work than doing it in a single thread in order.
