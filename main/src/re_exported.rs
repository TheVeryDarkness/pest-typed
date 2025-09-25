// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Re-export items from [`std`] (if feature `std` enabled), or [`core`] and [`alloc`] (if disabled).

/// `Box` in corresponding context.
#[cfg(not(feature = "std"))]
pub use ::alloc::boxed::Box;
/// `vec` in corresponding context.
#[cfg(not(feature = "std"))]
pub use ::alloc::vec;
/// `Vec` in corresponding context.
#[cfg(not(feature = "std"))]
pub use ::alloc::vec::Vec;
/// `Option` in corresponding context.
#[cfg(not(feature = "std"))]
pub use ::core::option::Option;
/// `Result` in corresponding context.
#[cfg(not(feature = "std"))]
pub use ::core::result::Result;
pub use ::core::{ops::FnMut, primitive::char};
#[cfg(feature = "std")]
pub use ::std::boxed::Box;
#[cfg(feature = "std")]
pub use ::std::option::Option;
#[cfg(feature = "std")]
pub use ::std::result::Result;
#[cfg(feature = "std")]
pub use ::std::vec;
#[cfg(feature = "std")]
pub use ::std::vec::Vec;
#[cfg(feature = "serde")]
pub use serde;
