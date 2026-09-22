use eventwise::{
    aggregate::{Aggregate, BootstrapError},
    event::{Event, StreamId},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub fn main() {
    println!("Hello, world!");
}

#[derive(Error, Debug)]
pub enum AccountError {
    #[error("insufficient funds")]
    InsufficientFunds,
}

struct Account {
    id: String,
    balance: f64,
}

#[derive(Clone)]
struct AccountId(Uuid);
impl StreamId for AccountId {
    fn key(&self) -> Uuid {
        self.0
    }
}
impl From<Uuid> for AccountId {
    fn from(uuid: Uuid) -> Self {
        AccountId(uuid)
    }
}

impl Aggregate for Account {
    const KIND: &'static str = "account";
    type Event = AccountEvent;
    type Id = AccountId;
    type Error = AccountError;

    fn create(event: &Self::Event) -> Result<Self, BootstrapError<Self::Event>> {
        match event {
            AccountEvent::AccountCreated {
                id,
                initial_balance,
            } => Ok(Account {
                id: id.clone(),
                balance: *initial_balance,
            }),
            _ => Err(BootstrapError(event.clone())),
        }
    }

    fn evolve(&mut self, event: &Self::Event) -> Result<(), Self::Error> {
        match event {
            AccountEvent::MoneyDeposited { amount } => {
                self.balance += amount;
                Ok(())
            }
            AccountEvent::MoneyWithdrawn { amount } => {
                self.balance -= amount;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum AccountEvent {
    AccountCreated { id: String, initial_balance: f64 },
    MoneyDeposited { amount: f64 },
    MoneyWithdrawn { amount: f64 },
}

impl Event for AccountEvent {
    fn kind(&self) -> &'static str {
        match self {
            AccountEvent::AccountCreated { .. } => "account_created",
            AccountEvent::MoneyDeposited { .. } => "money_deposited",
            AccountEvent::MoneyWithdrawn { .. } => "money_withdrawn",
        }
    }
}
