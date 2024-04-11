// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Definition of sequence-related macros and types.
//!
//! Sequences that are longer than 12 need be defined in your own crate with [crate::seq!].

#[macro_export]
/// Chained iterator type.
///
/// Used by [`crate::seq`].
macro_rules! chains {
    ($pest_typed:ident, $trait:ty, $t:ident, $T0:ty, ) => {
        <$T0 as $trait>::$t
    };
    ($pest_typed:ident, $trait:ty, $t:ident, $T0:ty, $($T:ty, )+) => {
        core::iter::Chain<<$T0 as $trait>::$t, $pest_typed::chains!($pest_typed, $trait, $t, $($T,)*)>
    };
}

#[macro_export]
/// Chained iterator.
///
/// Used by [`crate::seq`].
macro_rules! chain {
    ($pest_typed:ident, $trait:ty, $self: ident, iter, $T0:ty, $t0:tt, ) => {
        <$T0 as $trait>::iter_pairs(&$self.content.$t0)
    };
    ($pest_typed:ident, $trait:ty, $self: ident, iter, $T0:ty, $t0:tt, $($T:ty, $t:tt, )+) => {
        <$T0 as $trait>::iter_pairs(&$self.content.$t0).chain($pest_typed::chain!($pest_typed, $trait, $self, iter, $($T, $t, )*))
    };
    ($pest_typed:ident, $trait:ty, $self: ident, into_iter, $T0:ty, $t0:tt, ) => {
        <$T0 as $trait>::into_iter_pairs($self.content.$t0)
    };
    ($pest_typed:ident, $trait:ty, $self: ident, into_iter, $T0:ty, $t0:tt, $($T:ty, $t:tt, )+) => {
        <$T0 as $trait>::into_iter_pairs($self.content.$t0).chain($pest_typed::chain!($pest_typed, $trait, $self, into_iter, $($T, $t, )*))
    };
}

#[macro_export]
/// Generate sequences generics.
///
/// Also generate iterator type with [`crate::chains`] and [`crate::chain`].
macro_rules! seq {
    ($name:ident, $pest_typed:ident, $number:literal, $T0:ident, $t0:tt, $( $T:ident, $t:tt, )* ) => {
        #[doc = ::core::stringify!(Match a sequence with $number items.)]
        #[derive(Clone)]
        pub struct $name<$T0, $($T),*, > {
            #[doc = "Matched and skipped expressions."]
            pub content: ( $T0, $($T, )* ),
        }
        impl<$T0, $($T, )*> ::core::convert::From<( $T0, $($T, )* )>
            for $name<$T0, $($T, )*>
        {
            fn from(content: ( $T0, $($T, )* )) -> Self {
                Self { content }
            }
        }
        impl<
                'i,
                R: $pest_typed::RuleType,
                $T0: $pest_typed::TypedNode<'i, R>,
                $($T: $pest_typed::TypedNode<'i, R>, )*
                Skip: $pest_typed::NeverFailedTypedNode<'i, R> + ::core::default::Default,
                const SKIP: ::core::primitive::usize,
            > $pest_typed::TypedNode<'i, R> for $name<
                $pest_typed::predefined_node::Skipped<$T0, Skip, SKIP>,
                $(
                    $pest_typed::predefined_node::Skipped<$T, Skip, SKIP>,
                )*
            >
        {
            #[inline]
            fn try_parse_with(
                mut input: $pest_typed::Position<'i>,
                stack: &mut $pest_typed::Stack<$pest_typed::Span<'i>>,
                tracker: &mut $pest_typed::tracker::Tracker<'i, R>,
            ) -> ::core::result::Result<($pest_typed::Position<'i>, Self), ()> {
                let content =
                (
                    {
                        let skipped = ::core::array::from_fn(|_| Skip::default());
                        let (next, matched) = T0::try_parse_with(input, stack, tracker)?;
                        input = next;
                        $pest_typed::predefined_node::Skipped { skipped, matched }
                    },
                    $(
                        {
                            let skipped = ::core::array::from_fn(|_| {
                                let (next, skipped) = Skip::parse_with(input, stack);
                                input = next;
                                skipped
                            });
                            let (next, matched) = $T::try_parse_with(input, stack, tracker)?;
                            input = next;
                            $pest_typed::predefined_node::Skipped { skipped, matched }
                        },
                    )*
                );

                Ok((input, Self::from(content)))
            }
        }
        impl<
                'i: 'n,
                'n,
                R: $pest_typed::RuleType + 'n,
                $T0: $pest_typed::iterators::Pairs<'i, 'n, R>,
                $($T: $pest_typed::iterators::Pairs<'i, 'n, R>),*,
            > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<$T0, $($T, )*>
        {
            type Iter = $pest_typed::chains!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, Iter, $T0, $($T, )*);
            type IntoIter = $pest_typed::chains!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, IntoIter, $T0, $($T, )*);

            fn iter_pairs(&'n self) -> Self::Iter {
                $pest_typed::chain!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, self, iter, $T0, $t0, $($T, $t, )*)
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                $pest_typed::chain!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, self, into_iter, $T0, $t0, $($T, $t, )*)
            }
        }
        impl<$T0, $($T, )*> ::core::ops::Deref for $name<T0, $($T, )*> {
            type Target = ( T0, $($T, )* );
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<$T0, $($T, )*> ::core::ops::DerefMut for $name<T0, $($T, )*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
        impl<$T0: ::core::cmp::PartialEq, $($T: ::core::cmp::PartialEq, )*>
            ::core::cmp::PartialEq for $name<T0, $($T, )*>
        {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.content.$t0 == other.content.$t0
                $(
                    && self.content.$t == other.content.$t
                )*
            }
        }
        impl<$T0: ::core::cmp::Eq, $($T: ::core::cmp::Eq, )*>
            ::core::cmp::Eq for $name<T0, $($T, )*> {
        }
        impl<$T0: ::core::hash::Hash, $($T: ::core::hash::Hash, )*>
            ::core::hash::Hash for $name<T0, $($T, )*> {
            fn hash<H: ::core::hash::Hasher>(&self, hasher: &mut H) {
                ::core::hash::Hash::hash(&self.content.$t0, hasher);
                $(
                    ::core::hash::Hash::hash(&self.content.$t, hasher);
                )*
            }
        }
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> $name<$pest_typed::predefined_node::Skipped<$T0, IGNORED, SKIP>, $($pest_typed::predefined_node::Skipped<$T, IGNORED, SKIP>, )*> {
            /// Convert the reference of a sequence into a tuple of references of matched elements.
            pub fn as_ref(&self) -> ( &$T0, $(&$T, )* ) {
                self.get_matched()
            }
            /// Convert the reference of a sequence into a tuple of references of matched elements.
            pub fn get_matched(&self) -> ( &$T0, $(&$T, )* ) {
                ( &self.content.$t0.matched, $(&self.content.$t.matched, )* )
            }
            /// Convert a sequence into a tuple of matched elements.
            pub fn into_matched(self) -> ( $T0, $($T, )* ) {
                ( self.content.$t0.matched, $(self.content.$t.matched, )* )
            }
        }
        impl<$T0, $($T, )*> $name<T0, $($T, )*> {
            /// Convert the reference of a sequence into a tuple of references of skipped and matched elements.
            pub fn get_all(&self) -> ( &$T0, $(&$T, )* ) {
                ( &self.content.$t0, $(&self.content.$t, )* )
            }
            /// Convert a sequence into a tuple of skipped and matched elements.
            pub fn into_all(self) -> ( $T0, $($T, )* ) {
                ( self.content.$t0, $(self.content.$t, )* )
            }
        }
        impl<$T0, $($T, )*> ::core::convert::AsRef<( $T0, $($T, )* )> for $name<T0, $($T, )*> {
            fn as_ref(&self) -> &( $T0, $($T, )* ) {
                &self.content
            }
        }
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> ::core::convert::From<$name<$pest_typed::predefined_node::Skipped<$T0, IGNORED, SKIP>, $($pest_typed::predefined_node::Skipped<$T, IGNORED, SKIP>, )*>>
            for ( T0, $($T, )* )
        {
            fn from(value: $name<$pest_typed::predefined_node::Skipped<$T0, IGNORED, SKIP>, $($pest_typed::predefined_node::Skipped<$T, IGNORED, SKIP>, )*>) -> Self {
                ( value.content.$t0.matched, $(value.content.$t.matched, )* )
            }
        }
        impl<$T0, $($T, )*> ::core::convert::From<$name<T0, $($T, )*>>
            for ( T0, $($T, )* )
        {
            fn from(value: $name<T0, $($T, )*>) -> Self {
                ( value.content.$t0, $(value.content.$t, )* )
            }
        }
        impl<$T0: ::core::fmt::Debug, $($T: ::core::fmt::Debug, )*>
            ::core::fmt::Debug for $name<T0, $($T),*>
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(::core::stringify!($name))
                    .field(&self.content.$t0)
                    $(.field(&self.content.$t))*
                    .finish()
            }
        }
    };
}

seq!(Seq2, crate, 2, T0, 0, T1, 1,);
seq!(Seq3, crate, 3, T0, 0, T1, 1, T2, 2,);
seq!(Seq4, crate, 4, T0, 0, T1, 1, T2, 2, T3, 3,);
seq!(Seq5, crate, 5, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4,);
seq!(Seq6, crate, 6, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5,);
seq!(Seq7, crate, 7, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6,);
seq!(Seq8, crate, 8, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7,);
seq!(Seq9, crate, 9, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8,);
seq!(Seq10, crate, 10, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9,);
seq!(
    Seq11, crate, 11, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10,
    10,
);
seq!(
    Seq12, crate, 12, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10,
    10, T11, 11,
);
