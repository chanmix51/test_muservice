pub mod event;
pub mod model;
pub mod runtimes;
pub mod services;

pub type StdError = Box<dyn std::error::Error + Sync + Send>;
pub type StdResult<T> = Result<T, StdError>;
