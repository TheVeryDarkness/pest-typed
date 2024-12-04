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
/// Generate sequences generics.
///
/// Also generate iterator type with [`crate::chains`] and [`crate::chain`].
macro_rules! seq {
    ($name:ident, $number:literal, $T0:ident, $t0:tt, $( $T:ident, $t:tt, )* ) => {
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
                R: $crate::RuleType,
                $T0: $crate::TypedNode<'i, R>,
                $($T: $crate::TypedNode<'i, R>, )*
                Skip: $crate::NeverFailedTypedNode<'i, R> + ::core::default::Default,
                const SKIP: ::core::primitive::usize,
            > $crate::TypedNode<'i, R> for $name<
                $crate::predefined_node::Skipped<$T0, Skip, SKIP>,
                $(
                    $crate::predefined_node::Skipped<$T, Skip, SKIP>,
                )*
            >
        {
            #[inline]
            fn try_parse_partial_with<I: $crate::Input<'i>>(
                mut input: I,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::tracker::Tracker<'i, R>,
            ) -> ::core::option::Option<(I, Self)> {
                let content =
                (
                    {
                        let skipped = ::core::array::from_fn(|_| Skip::default());
                        let (next, matched) = T0::try_parse_partial_with(input, stack, tracker)?;
                        input = next;
                        $crate::predefined_node::Skipped { skipped, matched }
                    },
                    $(
                        {
                            let skipped = ::core::array::from_fn(|_| {
                                let (next, skipped) = Skip::parse_with(input, stack);
                                input = next;
                                skipped
                            });
                            let (next, matched) = $T::try_parse_partial_with(input, stack, tracker)?;
                            input = next;
                            $crate::predefined_node::Skipped { skipped, matched }
                        },
                    )*
                );

                Some((input, Self::from(content)))
            }
            #[inline]
            fn try_check_partial_with<I: $crate::Input<'i>>(
                mut input: I,
                stack: &mut $crate::Stack<$crate::Span<'i>>,
                tracker: &mut $crate::tracker::Tracker<'i, R>,
            ) -> ::core::option::Option<I> {
                {
                    let next = T0::try_check_partial_with(input, stack, tracker)?;
                    input = next;
                }
                $(
                    {
                        for _ in 0..SKIP {
                            let next = Skip::check_with(input, stack);
                            input = next;
                        }
                        let next = $T::try_check_partial_with(input, stack, tracker)?;
                        input = next;
                    }
                )*

                Some(input)
            }
        }
        impl<
                'i,
                R: $crate::RuleType,
                $T0: $crate::iterators::Pairs<'i, R>,
                $($T: $crate::iterators::Pairs<'i, R>),*,
            > $crate::iterators::Pairs<'i, R> for $name<$T0, $($T, )*>
        {
            fn for_self_or_each_child(&self, f: &mut impl $crate::re_exported::FnMut($crate::iterators::Token<'i, R>)) {
                self.content.0.for_self_or_each_child(f);
                $(
                    self.content.$t.for_self_or_each_child(f);
                )*
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
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> $name<$crate::predefined_node::Skipped<$T0, IGNORED, SKIP>, $($crate::predefined_node::Skipped<$T, IGNORED, SKIP>, )*> {
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
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> ::core::convert::From<$name<$crate::predefined_node::Skipped<$T0, IGNORED, SKIP>, $($crate::predefined_node::Skipped<$T, IGNORED, SKIP>, )*>>
            for ( T0, $($T, )* )
        {
            fn from(value: $name<$crate::predefined_node::Skipped<$T0, IGNORED, SKIP>, $($crate::predefined_node::Skipped<$T, IGNORED, SKIP>, )*>) -> Self {
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

seq!(Seq2, 2, T0, 0, T1, 1,);
seq!(Seq3, 3, T0, 0, T1, 1, T2, 2,);
seq!(Seq4, 4, T0, 0, T1, 1, T2, 2, T3, 3,);
seq!(Seq5, 5, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4,);
seq!(Seq6, 6, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5,);
seq!(Seq7, 7, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6,);
seq!(Seq8, 8, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7,);
seq!(Seq9, 9, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8,);
seq!(Seq10, 10, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9,);
seq!(Seq11, 11, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10,);
seq!(
    Seq12, 12, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11,
    11,
);
