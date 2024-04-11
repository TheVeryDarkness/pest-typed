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
    ($pest_typed:ident, $name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, $V1:ident, $v1:tt, $( $V:ident, $v:tt, )* ) => {
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

        impl<Ret, $V0, $V1, $($V, )*> $pest_typed::choices::NextChoice for $v0<Ret, $V0, $V1, $($V, )*> {
            type Next = $v1<Ret, $V1, $($V, )*>;
        }
        impl<Ret, $V0, $V1, $($V, )*> $v0<Ret, $V0, $V1, $($V, )*>
        {
            pub fn else_if(self, f: impl FnOnce($V0) -> Ret) -> <Self as $pest_typed::choices::NextChoice>::Next {
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
        $crate::choices_helper!($pest_typed, $name, ($_V0, $_v0, $( $_V, $_v, )* ), $V1, $v1, $( $V, $v, )* );
    };
    ($pest_typed:ident, $name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, ) => {
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
/// Generate an iterator type that implements [`core::iter::Iterator`] for choices.
/// Automatically called by [`crate::choices!`].
macro_rules! choices_iter {
    ($name:ident, $pest_typed:ident, $iter_type:ident, $iter_func:ident, $item:ty, $V0:ident, $v0:tt, $( $V:ident, $v:tt, )* ) => {
        pub enum $iter_type<'i, 'n, R: $pest_typed::RuleType, $V0: $pest_typed::iterators::Pairs<'i, 'n, R>, $($V: $pest_typed::iterators::Pairs<'i, 'n, R>, )* > {
            $v0($V0::$iter_type),
            $(
                $v($V::$iter_type),
            )*
        }
        impl<'i: 'n, 'n, R: $pest_typed::RuleType + 'n, $V0: $pest_typed::iterators::Pairs<'i, 'n, R>, $($V: $pest_typed::iterators::Pairs<'i, 'n, R>, )* >
            ::core::iter::Iterator for $iter_type<'i, 'n, R, $V0, $($V, )*>
        {
            type Item = $item;
            fn next(&mut self) -> Option<<Self as ::core::iter::Iterator>::Item> {
                match self {
                    Self::$v0($v0) => $v0.next(),
                    $(Self::$v($v) => $v.next(), )*
                }
            }
        }
    };
}

#[macro_export]
/// Generate choices with given type name and variant names.
///
/// Also traverse helpers by calling [`choices_helper`] and iterators by calling [`choices_iter`].
macro_rules! choices {
    ($name:ident, $pest_typed:ident, $mod:ident, $number:literal, $V0:ident, $v0:tt, $( $V:ident, $v:tt, )* ) => {
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
                pub fn reference<'n, Ret>(&'n self) -> helper::$v0<Ret, &'n $V0, $(&'n $V, )*> {
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
                pub fn $v0(&self) -> ::core::option::Option<&$V0> {
                    if let Self::$v0(res) = self {
                        ::core::option::Option::Some(res)
                    } else {
                        ::core::option::Option::None
                    }
                }
                $(
                    /// Access inner node if matched.
                    pub fn $v(&self) -> ::core::option::Option<&$V> {
                        if let Self::$v(res) = self {
                            ::core::option::Option::Some(res)
                        } else {
                            ::core::option::Option::None
                        }
                    }
                )*
            }
            impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* > $pest_typed::TypedNode<'i, R>
                for $name<$V0, $($V, )* >
            {
                #[inline]
                fn try_parse_with(
                    input: $pest_typed::Position<'i>,
                    stack: &mut $pest_typed::Stack<$pest_typed::Span<'i>>,
                    tracker: &mut $pest_typed::tracker::Tracker<'i, R>,
                ) -> ::core::option::Option<($pest_typed::Position<'i>, Self)> {
                    let res = $pest_typed::predefined_node::restore_on_none(stack, |stack| $V0::try_parse_with(input, stack, tracker));
                    if let Some((input, res)) = res {
                        return Some((input, Self::$v0(res)));
                    }
                    $(
                        let res = $pest_typed::predefined_node::restore_on_none(stack, |stack| $V::try_parse_with(input, stack, tracker));
                        if let Some((input, res)) = res {
                            return Some((input, Self::$v(res)));
                        }
                    )*
                    None
                }
            }
            impl<
                'i: 'n,
                'n,
                R: $pest_typed::RuleType + 'i,
                $V0: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
                $($V: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>, )*
            > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<$V0, $($V, )* >
            {
                type Iter = iterators::Iter<'i, 'n, R, $V0, $($V, )*>;
                type IntoIter = iterators::IntoIter<'i, 'n, R, $V0, $($V, )*>;

                fn iter_pairs(&'n self) -> Self::Iter {
                    match self {
                        Self::$v0($v0) => Self::Iter::$v0($v0.iter_pairs()),
                        $(
                            Self::$v($v) => Self::Iter::$v($v.iter_pairs()),
                        )*
                    }
                }
                fn into_iter_pairs(self) -> Self::IntoIter {
                    match self {
                        Self::$v0($v0) => Self::IntoIter::$v0($v0.into_iter_pairs()),
                        $(Self::$v($v) => Self::IntoIter::$v($v.into_iter_pairs()), )*
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
                $crate::choices_helper!($pest_typed, $name, ($V0, $v0, $( $V, $v, )* ), $V0, $v0, $( $V, $v, )* );
            }
            mod iterators {
                $crate::choices_iter!($name, $pest_typed, Iter, iter, &'n (dyn $pest_typed::iterators::Pair<'i, 'n, R>), $V0, $v0, $( $V, $v, )* );
                $crate::choices_iter!($name, $pest_typed, IntoIter, into_iter, $pest_typed::Box<dyn $pest_typed::iterators::Pair<'i, 'n, R> + 'n>, $V0, $v0, $( $V, $v, )* );
            }
        }
    };
}

// Choices helper and iterator.

choices!(Choice2, crate, choice2, 2, T0, _0, T1, _1,);
choices!(Choice3, crate, choice3, 3, T0, _0, T1, _1, T2, _2,);
choices!(Choice4, crate, choice4, 4, T0, _0, T1, _1, T2, _2, T3, _3,);
choices!(Choice5, crate, choice5, 5, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4,);
choices!(Choice6, crate, choice6, 6, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5,);
choices!(Choice7, crate, choice7, 7, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,);
choices!(
    Choice8, crate, choice8, 8, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
);
choices!(
    Choice9, crate, choice9, 9, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7, T8,
    _8,
);
choices!(
    Choice10, crate, choice10, 10, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
    T8, _8, T9, _9,
);
choices!(
    Choice11, crate, choice11, 11, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
    T8, _8, T9, _9, T10, _10,
);
choices!(
    Choice12, crate, choice12, 12, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
    T8, _8, T9, _9, T10, _10, T11, _11,
);
