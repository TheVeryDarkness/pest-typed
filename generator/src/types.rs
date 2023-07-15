// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use proc_macro2::TokenStream;
use quote::quote;

pub fn box_type() -> TokenStream {
    #[cfg(feature = "std")]
    quote! { ::std::boxed::Box }

    #[cfg(not(feature = "std"))]
    quote! { ::alloc::boxed::Box }
}

pub fn result_type() -> TokenStream {
    #[cfg(feature = "std")]
    quote! { ::std::result::Result }

    #[cfg(not(feature = "std"))]
    quote! { ::core::result::Result }
}

pub fn option_type() -> TokenStream {
    #[cfg(feature = "std")]
    quote! { ::std::option::Option }

    #[cfg(not(feature = "std"))]
    quote! { ::core::option::Option }
}
