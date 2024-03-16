pub use err::*;
pub use fetch::*;

pub mod endpoints;
mod err;
mod fetch;
pub mod vanilla;
#[cfg(test)]
mod test;
mod ir;

pub type Result<T, E = Error> = std::result::Result<T, E>;
