// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Definition of choices-related macros and types.
//!
//! Choices with more than 12 branches should be defined in your own crate with [`crate::choices!`].

/// Choice helper with a next branch.
pub trait NextChoice {
    /// The choice helper that corresponds to the next branch.
    type Next;
}

#[macro_export]
/// Generate choice helpers that can be used to traverse those branches.
/// Automatically called by [`crate::choices!`].
macro_rules! choices_helper {
    ($name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, $V1:ident, $v1:tt, $( $V:ident, $v:tt, )* ) => {
        /// Choices.
        /// - `Ret`: Return value type.
        /// - `...`: Choices branches.
        pub enum $v0<Ret, $V0, $V1, $($V, )*> {
            $v0($V0),
            $v1($V1),
            $(
                $v($V),
            )*
            Res(Ret),
        }

        impl<Ret, $V0, $V1, $($V, )*> $crate::choices::NextChoice for $v0<Ret, $V0, $V1, $($V, )*> {
            type Next = $v1<Ret, $V1, $($V, )*>;
        }
        impl<Ret, $V0, $V1, $($V, )*> $v0<Ret, $V0, $V1, $($V, )*>
        {
            pub fn else_if(self, f: impl FnOnce($V0) -> Ret) -> <Self as $crate::choices::NextChoice>::Next {
                match self {
                    Self::$v0(c) => $v1::Res(f(c)),
                    Self::$v1(c) => $v1::$v1(c),
                    $(
                        Self::$v(c) => $v1::$v(c),
                    )*
                    Self::Res(res) => $v1::Res(res),
                }
            }
        }
        $crate::choices_helper!($name, ($_V0, $_v0, $( $_V, $_v, )* ), $V1, $v1, $( $V, $v, )* );
    };
    ($name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, ) => {
        pub enum $v0<Ret, $V0> {
            $v0($V0),
            Res(Ret),
        }
        impl<Ret, $V0> $v0<Ret, $V0> {
            pub fn else_then(self, f: impl FnOnce($V0) -> Ret) -> Ret {
                match self {
                    Self::$v0(c) => f(c),
                    Self::Res(res) => res,
                }
            }
        }
    }
}

#[macro_export]
/// Generate choices with given type name and variant names.
///
/// Also traverse helpers by calling [`choices_helper`] and iterators by calling [`choices_iter`].
macro_rules! choices {
    ($name:ident, $mod:ident, $number:literal, $V0:ident, $v0:tt, $( $V:ident, $v:tt, )* ) => {
        pub use $mod::$name;
        #[doc = ::core::stringify!(Types for choices type [$name].)]
        pub mod $mod {
            #![allow(clippy::style)]

            #[doc = ::core::stringify!(Match one of $number expressions.)]
            #[derive(Clone, Hash, PartialEq, Eq)]
            pub enum $name<$V0, $($V, )* > {
                #[doc = ::core::stringify!(Variant $v0.)]
                $v0($V0),
                $(
                    #[doc = ::core::stringify!(Variant $v.)]
                    $v($V),
                )*
            }
            impl<$V0, $($V, )* > $name<$V0, $($V, )* > {
                /// Traverse all branches with reference.
                #[allow(clippy::needless_lifetimes)]
                pub const fn reference<'n, Ret>(&'n self) -> helper::$v0<Ret, &'n $V0, $(&'n $V, )*> {
                    match self {
                        Self::$v0(c) => helper::$v0::$v0(c),
                        $(
                            Self::$v(c) => helper::$v0::$v(c),
                        )*
                    }
                }
                /// Invoke if is not None and is the first case.
                pub fn if_then<'n, Ret>(&'n self, f: impl FnOnce(&'n $V0) -> Ret) -> <helper::$v0<Ret, &'n $V0, $(&'n $V, )* > as $crate::choices::NextChoice>::Next {
                    self.reference().else_if(f)
                }
                /// Traverse all branches with reference.
                pub fn consume<Ret>(self) -> helper::$v0<Ret, $V0, $($V, )*> {
                    match self {
                        Self::$v0(c) => helper::$v0::$v0(c),
                        $(
                            Self::$v(c) => helper::$v0::$v(c),
                        )*
                    }
                }
                /// Invoke if is not None and is the first case.
                pub fn consume_if_then<Ret>(self, f: impl FnOnce($V0) -> Ret) -> <helper::$v0<Ret, $V0, $($V, )* > as $crate::choices::NextChoice>::Next {
                    self.consume().else_if(f)
                }
                /// Access inner node if matched.
                pub const fn $v0(&self) -> ::core::option::Option<&$V0> {
                    if let Self::$v0(res) = self {
                        ::core::option::Option::Some(res)
                    } else {
                        ::core::option::Option::None
                    }
                }
                $(
                    /// Access inner node if matched.
                    pub const fn $v(&self) -> ::core::option::Option<&$V> {
                        if let Self::$v(res) = self {
                            ::core::option::Option::Some(res)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                )*
            }
            impl<C: $crate::Cursor, R: $crate::RuleType, $V0: $crate::TypedNode<C, R>, $($V: $crate::TypedNode<C, R>, )* > $crate::TypedNode<C, R>
                for $name<$V0, $($V, )* >
            {
                #[inline]
                fn try_parse_partial_with(
                    input: C,
                    stack: &mut $crate::Stack<$crate::Span<C::String>>,
                    tracker: &mut $crate::tracker::Tracker<C::String, R>,
                ) -> ::core::option::Option<(C, Self)> {
                    let res = $crate::predefined_node::restore_on_none(stack, |stack| $V0::try_parse_partial_with(input.clone(), stack, tracker));
                    if let Some((input, res)) = res {
                        return Some((input, Self::$v0(res)));
                    }
                    $(
                        let res = $crate::predefined_node::restore_on_none(stack, |stack| $V::try_parse_partial_with(input.clone(), stack, tracker));
                        if let Some((input, res)) = res {
                            return Some((input, Self::$v(res)));
                        }
                    )*
                    None
                }

                #[inline]
                fn try_check_partial_with(
                    input: C,
                    stack: &mut $crate::Stack<$crate::Span<C::String>>,
                    tracker: &mut $crate::tracker::Tracker<C::String, R>,
                ) -> ::core::option::Option<C> {
                    let res = $crate::predefined_node::restore_on_none(stack, |stack| $V0::try_check_partial_with(input.clone(), stack, tracker));
                    if let Some(input) = res {
                        return Some(input);
                    }
                    $(
                        let res = $crate::predefined_node::restore_on_none(stack, |stack| $V::try_check_partial_with(input.clone(), stack, tracker));
                        if let Some(input) = res {
                            return Some(input);
                        }
                    )*
                    None
                }
            }
            impl<
                S: $crate::RefStr,
                R: $crate::RuleType,
                $V0: $crate::iterators::Pairs<S, R>,
                $($V: $crate::iterators::Pairs<S, R>, )*
            > $crate::iterators::Pairs<S, R> for $name<$V0, $($V, )* >
            {
                fn for_self_or_each_child(&self, f: &mut impl FnMut($crate::iterators::Token<S, R>)) {
                    match self {
                        Self::$v0($v0) =>$v0.for_self_or_each_child(f),
                        $(
                            Self::$v($v) => $v.for_self_or_each_child(f),
                        )*
                    }
                }
            }
            impl<$V0: ::core::fmt::Debug, $($V: ::core::fmt::Debug, )* >
                ::core::fmt::Debug for $name<$V0, $($V, )* >
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Self::$v0($v0) => f.debug_struct(::core::stringify!($name)).field(&::core::stringify!($v0), &$v0).finish(),
                        $(
                            Self::$v($v) => f.debug_struct(::core::stringify!($name)).field(&::core::stringify!($v), &$v).finish(),
                        )*
                    }
                }
            }
            mod helper {
                $crate::choices_helper!($name, ($V0, $v0, $( $V, $v, )* ), $V0, $v0, $( $V, $v, )* );
            }
        }
    };
}

// Choices helper and iterator.

choices!(Choice2, choice2, 2, T0, _0, T1, _1,);
choices!(Choice3, choice3, 3, T0, _0, T1, _1, T2, _2,);
choices!(Choice4, choice4, 4, T0, _0, T1, _1, T2, _2, T3, _3,);
choices!(Choice5, choice5, 5, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4,);
choices!(Choice6, choice6, 6, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5,);
choices!(Choice7, choice7, 7, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,);
choices!(Choice8, choice8, 8, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,);
choices!(
    Choice9, choice9, 9, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7, T8, _8,
);
choices!(
    Choice10, choice10, 10, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7, T8, _8,
    T9, _9,
);
choices!(
    Choice11, choice11, 11, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7, T8, _8,
    T9, _9, T10, _10,
);
choices!(
    Choice12, choice12, 12, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7, T8, _8,
    T9, _9, T10, _10, T11, _11,
);
