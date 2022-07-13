#![ cfg_attr( not( feature = "use_std" ), no_std ) ]
#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/data_type/latest/wmath/" ) ]
#![ warn( rust_2018_idioms ) ]
#![ warn( missing_debug_implementations ) ]
#![ warn( missing_docs ) ]

// #![ feature( trait_alias ) ]
// #![ feature( generic_associated_types ) ]
// #![ feature( type_name_of_val ) ]
// #![ feature( generic_associated_types ) ]

//!
//! Math library with adapters.
//!

#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/Readme.md" ) ) ]

use meta_tools::mod_interface;

pub mod dependency
{
  pub use math_adapter;
}

pub use math_adapter as adapter;
pub use adapter::exposed::*;

mod_interface!
{

  layer basic;

  // xxx : broken
  // use super::adapter;

}
