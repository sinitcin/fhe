

// 
//  ! CryptoContextImpl
// 
//  A CryptoContextImpl is the object used to access the OpenFHE library
// 
//  All OpenFHE functionality is accessed by way of an instance of a
//  CryptoContextImpl; we say that various objects are "created in" a context,
//  and can only be used in the context in which they were created
// 
//  All OpenFHE methods are accessed through CryptoContextImpl methods. Guards
//  are implemented to make certain that only valid objects that have been
//  created in the context are used
// 
//  Contexts are created using GenCryptoContext(), and can be serialized
//  and recovered from a serialization
// 