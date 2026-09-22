use thiserror::Error;

use crate::aggregate::Aggregate;

// pub trait Handle<C: Command> {
//     fn handle(&self, cmd: &C

#[derive(Error, Debug)]
pub enum CommandError<E: std::error::Error> {
    #[error("aggregate not initialized by bootstrap event")]
    NotInitialized,
    #[error("domain error: {0}")]
    Domain(E),
}

pub trait Bootstrap<C>: Aggregate {}

pub trait Handle<C>: Aggregate {
    fn handle(&self, cmd: &C) -> Result<Vec<Self::Event>, Self::Error>;
    fn bootstrap(cmd: &C) -> Result<Vec<Self::Event>, CommandError<Self::Error>> {
        Err(CommandError::NotInitialized)
    }
}
