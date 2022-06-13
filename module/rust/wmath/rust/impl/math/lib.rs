#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( generic_associated_types ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( generic_associated_types ) ]

//!
//! .Math library with adapters.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

/// Basics.
pub mod basic;

/// Namespace with dependencies.
pub mod dependency
{
  pub use math_adapter;
}

pub use math_adapter as adapter;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::basic::exposed::*;
  pub use super::adapter::exposed::*;
}

pub use exposed::*;

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  pub use super::basic::prelude::*;
  pub use super::adapter::prelude::*;
}
