// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_export]
macro_rules! chains {
    ($pest_typed:ident, $t:ident, $T0:ident, ) => {
        $T0::$t
    };
    ($pest_typed:ident, $t:ident, $T0:ident, $($T:ident, )+) => {
        core::iter::Chain<$T0::$t, $pest_typed::chains!($pest_typed, $t, $($T,)*)>
    };
}

#[macro_export]
macro_rules! chain {
    ($pest_typed:ident, $self: ident, $f: ident, $t0:tt, ) => {
        $self.content.$t0.$f()
    };
    ($pest_typed:ident, $self: ident, $f: ident, $t0:tt, $($t:tt, )+) => {
        $self.content.$t0.$f().chain($pest_typed::chain!($pest_typed, $self, $f, $($t, )*))
    };
}
