use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

#[allow(unused)]
pub type Result<T> = core::result::Result<T, Error>;
