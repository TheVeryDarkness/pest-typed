// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Wrappers of constants and types, so that they can be used in generics easier.

use crate::RuleType;

/// An object containing a constant.
pub trait Storage<T> {
    /// Get contained string.
    fn get_content(&self) -> T;
}
/// An object containing a constant.
pub trait ConstantStorage<T> {
    /// Get contained string with `self`.
    fn get_constant(&self) -> T;
}

/// A wrapper for string as a generics argument.
pub trait StringWrapper {
    /// Wrapped string.
    const CONTENT: &'static str;
}
impl<T: StringWrapper> Storage<&'static str> for T {
    fn get_content(&self) -> &'static str {
        Self::CONTENT
    }
}
impl<T: StringWrapper> ConstantStorage<&'static str> for T {
    fn get_constant(&self) -> &'static str {
        Self::CONTENT
    }
}

/// A wrapper for string array as a generics argument.
pub trait StringArrayWrapper {
    /// Wrapped strings.
    const CONTENT: &'static [&'static str];
}
impl<T: StringArrayWrapper> Storage<&'static [&'static str]> for T {
    fn get_content(&self) -> &'static [&'static str] {
        Self::CONTENT
    }
}

/// Rule wrapper.
pub trait RuleWrapper<R: RuleType> {
    /// Wrapped rule.
    const RULE: R;
    /// The type of wrapped rule.
    type Rule;

    /// Get wrapped rule.
    fn get_rule(&self) -> R {
        Self::RULE
    }
}

/// Type wrapper.
pub trait TypeWrapper {
    /// Wrapped type.
    type Inner;
}

/// Bound for the length of vector.
pub trait BoundWrapper {
    /// Min length of a vector.
    const MIN: usize;
    /// Max length of a vector.
    const MAX: usize;

    /// Get min length.
    fn get_min_len(&self) -> usize {
        Self::MIN
    }
    /// Get max length.
    fn get_max_len(&self) -> usize {
        Self::MAX
    }
}
