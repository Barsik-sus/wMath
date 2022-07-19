//!
//! Abstractions.
//!

meta_tools::mod_interface!
{

  layer as_foreign;
  layer from;
  layer nan_like;

  #[ cfg( feature = "use_std" ) ]
  layer x2_interface;
  // layer x3_interface;
  // xxx

  // layer box2_interface;
  // layer box3_interface;

}
