use std::marker::PhantomData;

use thiserror::Error;

use crate::aggregate::Aggregate;

// pub trait Handle<C: Command> {
//     fn handle(&self, cmd: &C

#[derive(Error, Debug)]
pub enum CommandError<E: std::error::Error> {
    #[error("aggregate not initialized by bootstrap event")]
    NotInitialized,
    #[error("command not supported")]
    NotSupported,
    #[error("domain error: {0}")]
    Domain(E),
}

pub trait Handle<C, R = ()>: Aggregate {
    fn handle(&self, _cmd: C) -> Result<(R, Vec<Self::Event>), CommandError<Self::Error>> {
        Err(CommandError::NotSupported)
    }
    fn create(_cmd: C) -> Result<(R, Vec<Self::Event>), CommandError<Self::Error>> {
        Err(CommandError::NotInitialized)
    }
}
