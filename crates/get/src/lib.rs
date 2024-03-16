pub use err::*;
pub use fetch::*;

pub mod endpoints;
mod err;
mod fetch;
mod ir;
#[cfg(test)]
mod test;
pub mod vanilla;

pub type Result<T, E = Error> = std::result::Result<T, E>;
