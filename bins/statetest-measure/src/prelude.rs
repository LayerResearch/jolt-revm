//! Custom prelude for no_std environments
//!
//! Import this in each module with: `use crate::prelude::*;`

#[cfg(not(feature = "std"))]
pub use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(not(feature = "std"))]
pub use core::{
    option::Option::{self, None, Some},
    result::Result::{self, Err, Ok},
};
