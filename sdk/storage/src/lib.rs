#![recursion_limit = "256"]
#![allow(clippy::needless_lifetimes)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;
#[macro_use]
extern crate quick_error;

#[cfg(feature = "account")]
pub mod account;
#[cfg(feature = "adls_gen2")]
pub mod adls_gen2;
#[cfg(feature = "blob")]
pub mod blob;
pub mod core;
#[cfg(feature = "queue")]
pub mod queue;
#[cfg(feature = "table")]
pub mod table;

pub mod clients;

pub use crate::core::*;
#[cfg(feature = "account")]
pub use account::*;
#[cfg(feature = "adls_gen2")]
pub use adls_gen2::*;
#[cfg(feature = "blob")]
pub use blob::*;
#[cfg(feature = "queue")]
pub use queue::*;
#[cfg(feature = "table")]
pub use table::*;
