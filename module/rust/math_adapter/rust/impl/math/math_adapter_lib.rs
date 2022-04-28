#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( generic_associated_types ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( generic_associated_types ) ]

//!
//! Collection of math adapters to decouple your application from math libraries' implementations and to increase inter-libraries compatibility / exchangeability.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// General math traits.
pub use num_traits as traits;

/// Implementation of adapters for specific math libraries.
pub mod plugin;
/// Define scalar interface.
pub mod scalar;
/// Adapters.
pub mod vector;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::plugin::exposed::*;
  pub use super::scalar::exposed::*;
  pub use super::vector::exposed::*;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  pub use super::plugin::prelude::*;
  pub use super::scalar::prelude::*;
  pub use super::vector::prelude::*;
}