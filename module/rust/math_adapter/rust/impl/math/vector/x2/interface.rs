
///
/// Nominal interface of vector X2.
///

#[ allow( non_camel_case_types ) ]
pub trait X2NominalInterface
{
  /// Type of element.
  type Scalar : ScalarInterface;

  // /// Constructor.
  // fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self;

  /// First element.
  fn _0( &self ) -> Self::Scalar;
  /// Second element.
  fn _1( &self ) -> Self::Scalar;
  /// First element.
  #[ inline ]
  fn x( &self ) -> Self::Scalar
  {
    self._0()
  }
  /// Second element.
  #[ inline ]
  fn y( &self ) -> Self::Scalar
  {
    self._1()
  }

  /// Clone as tuple.
  #[ inline ]
  fn clone_as_tuple( &self ) -> ( Self::Scalar , Self::Scalar )
  {
    ( self._0(), self._1() )
  }
  /// Clone as array.
  #[ inline ]
  fn clone_as_array( &self ) -> [ Self::Scalar ; 2 ]
  {
    [ self._0(), self._1() ]
  }
  /// Clone as canonical.
  #[ inline ]
  fn clone_as_canonical( &self ) -> X2< Self::Scalar >
  {
    X2::< Self::Scalar >( self._0(), self._1() )
  }

}

///
/// Standard interface of vector X2. Implements nominal interface, extending it by constructor `make`.
///

#[ allow( non_camel_case_types ) ]
pub trait X2BasicInterface : X2NominalInterface
{

  /// Constructor.
  fn make( _0 : Self::Scalar, _1 : Self::Scalar ) -> Self;

}

///
/// Interface of vector X2 for structures with the canonical layout.
///

#[ allow( non_camel_case_types ) ]
pub trait X2CanonicalInterface : X2BasicInterface + Sized
{
  /// Assign value.
  #[ inline ]
  fn assign< Src : X2BasicInterface< Scalar = Self::Scalar > >( &mut self, src : Src )
  {
    *self._0_mut() = src._0();
    *self._1_mut() = src._1();
  }

  /// First element.
  fn _0_ref( &self ) -> &Self::Scalar;
  /// Second element.
  fn _1_ref( &self ) -> &Self::Scalar;
  /// First element.
  #[ inline ]
  fn x_ref( &self ) -> &Self::Scalar
  {
    self._0_ref()
  }
  /// Second element.
  #[ inline ]
  fn y_ref( &self ) -> &Self::Scalar
  {
    self._1_ref()
  }

  /// First element.
  fn _0_mut( &mut self ) -> &mut Self::Scalar;
  /// Second element.
  fn _1_mut( &mut self ) -> &mut Self::Scalar;
  /// First element.
  #[ inline ]
  fn x_mut( &mut self ) -> &mut Self::Scalar
  {
    self._0_mut()
  }
  /// Second element.
  #[ inline ]
  fn y_mut( &mut self ) -> &mut Self::Scalar
  {
    self._1_mut()
  }

  /// Interpret as tuple.
  #[ inline ]
  fn as_tuple( &self ) -> &( Self::Scalar , Self::Scalar )
  {
    debug_assert_eq!( core::mem::size_of::< Self >(), core::mem::size_of::< ( Self::Scalar , Self::Scalar ) >() );
    unsafe
    {
      std::mem::transmute::< _, _ >( self.as_canonical() )
    }
  }
  /// Interpret as array.
  #[ inline ]
  fn as_array( &self ) -> &[ Self::Scalar ; 2 ]
  {
    unsafe
    {
      std::mem::transmute::< _, _ >( self.as_canonical() )
    }
  }
  /// Interpret as slice.
  #[ inline ]
  fn as_slice( &self ) -> &[ Self::Scalar ]
  {
    &self.as_array()[ .. ]
  }
  /// Canonical representation of the vector.
  fn as_canonical( &self ) -> &X2< Self::Scalar >;

  /// Interpret as mutable tuple.
  #[ inline ]
  fn as_tuple_mut( &mut self ) -> &mut ( Self::Scalar , Self::Scalar )
  {
    debug_assert_eq!( core::mem::size_of::< Self >(), core::mem::size_of::< ( Self::Scalar , Self::Scalar ) >() );
    unsafe
    {
      std::mem::transmute::< _, _ >( self.as_canonical_mut() )
    }
  }
  /// Interpret as mutable array.
  #[ inline ]
  fn as_array_mut( &mut self ) -> &mut [ Self::Scalar ; 2 ]
  {
    unsafe
    {
      std::mem::transmute::< _, _ >( self.as_canonical_mut() )
    }
  }
  /// Interpret as mutable slice.
  #[ inline ]
  fn as_slice_mut( &mut self ) -> &mut [ Self::Scalar ]
  {
    &mut self.as_array_mut()[ .. ]
  }
  /// Mutable canonical representation of the vector.
  fn as_canonical_mut( &mut self ) -> &mut X2< Self::Scalar >;

}

/* zzz : index of methods? */

//

impl< Scalar, Original, Target > crate::From2< Original >
for Target
where
  Scalar : ScalarInterface,
  Original : X2NominalInterface< Scalar = Scalar >,
  Target : X2BasicInterface< Scalar = Scalar >,
{
  fn from2( original : Original ) -> Self
  {
    Self::make( original._0(), original._1() )
  }
}
