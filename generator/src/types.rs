// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Extracted from **pest/generator/src/generator.rs** (commit ac0aed3eecf435fd93ba575a39704aaa88a375b7)
//! and modified, but re-written then.
//!
//! It's for convenient use of types in standard library whether or not feature **std** is enabled.

use proc_macro2::TokenStream;
use quote::quote;

/// `Box` in corresponding context.
pub fn box_type() -> TokenStream {
    quote! { ::pest_typed::re_exported::Box }
}

/// `Result` in corresponding context.
pub fn result_type() -> TokenStream {
    quote! { ::pest_typed::re_exported::Result }
}

/// `Option` in corresponding context.
pub fn option_type() -> TokenStream {
    quote! { ::pest_typed::re_exported::Option }
}

/// `Vec` in corresponding context.
pub fn vec_type() -> TokenStream {
    quote! { ::pest_typed::re_exported::Vec }
}
