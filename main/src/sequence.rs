// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Definition of sequence-related macros and types.

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
    ($pest_typed:ident, $trait:ty, $self: ident, iter, $Skipped:ty, $T0:ty, $t0:tt, ) => {
        <($Skipped, $T0) as $trait>::iter_pairs(&$self.content.$t0)
    };
    ($pest_typed:ident, $trait:ty, $self: ident, iter, $Skipped:ty, $T0:ty, $t0:tt, $($T:ty, $t:tt, )+) => {
        <($Skipped, $T0) as $trait>::iter_pairs(&$self.content.$t0).chain($pest_typed::chain!($pest_typed, $trait, $self, iter, $Skipped, $($T, $t, )*))
    };
    ($pest_typed:ident, $trait:ty, $self: ident, into_iter, $Skipped:ty, $T0:ty, $t0:tt, ) => {
        <($Skipped, $T0) as $trait>::into_iter_pairs($self.content.$t0)
    };
    ($pest_typed:ident, $trait:ty, $self: ident, into_iter, $Skipped:ty, $T0:ty, $t0:tt, $($T:ty, $t:tt, )+) => {
        <($Skipped, $T0) as $trait>::into_iter_pairs($self.content.$t0).chain($pest_typed::chain!($pest_typed, $trait, $self, into_iter, $Skipped, $($T, $t, )*))
    };
}

#[macro_export]
/// Generate sequences generics.
///
/// Also generate iterator type with [`crate::chain`] and [`crate::chain`].
macro_rules! seq {
    ($name:ident, $pest_typed:ident, $number:literal, $T0:ident, $t0:tt, $( $T:ident, $t:tt, )* ) => {
        #[doc = ::core::stringify!(Match a sequence with $number items.)]
        #[derive(Clone)]
        pub struct $name<$T0, $($T),*, const SKIP: usize, IGNORED, > {
            #[doc = "Matched and skipped expressions."]
            pub content: ( ([IGNORED; SKIP], $T0), $(([IGNORED; SKIP], $T), )* ),
        }
        impl<$T0, $($T, )* const SKIP: usize, IGNORED> ::core::convert::From<( ([IGNORED; SKIP], $T0), $(([IGNORED; SKIP], $T), )* )>
            for $name<$T0, $($T),*, SKIP, IGNORED>
        {
            fn from(content: ( ([IGNORED; SKIP], $T0), $(([IGNORED; SKIP], $T), )* )) -> Self {
                Self { content }
            }
        }
        impl<
                'i,
                R: $pest_typed::RuleType,
                $T0: $pest_typed::TypedNode<'i, R>,
                $($T: $pest_typed::TypedNode<'i, R>, )*
                const SKIP: usize,
                IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
            > $pest_typed::TypedNode<'i, R> for $name<$T0, $($T),*, SKIP, IGNORED>
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
                        let (next, content) = T0::try_parse_with(input, stack, tracker)?;
                        input = next;
                        (::core::array::from_fn(|_| IGNORED::default()), content)
                    },
                    $(
                        {
                            let skipped = ::core::array::from_fn(|_| {
                                let (next, skipped) = IGNORED::parse_with(input, stack);
                                input = next;
                                skipped
                            });
                            let (next, content) = $T::try_parse_with(input, stack, tracker)?;
                            input = next;
                            (skipped, content)
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
                $T0: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
                $($T: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>),*,
                const SKIP: usize,
                IGNORED: $pest_typed::NeverFailedTypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R> + 'n,
            > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<$T0, $($T),*, SKIP, IGNORED>
        {
            type Iter = $pest_typed::chains!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, Iter, ([IGNORED; SKIP], $T0), $(([IGNORED; SKIP], $T), )*);
            type IntoIter = $pest_typed::chains!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, IntoIter, ([IGNORED; SKIP], $T0), $(([IGNORED; SKIP], $T), )*);

            fn iter_pairs(&'n self) -> Self::Iter {
                $pest_typed::chain!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, self, iter, [IGNORED; SKIP], $T0, $t0, $($T, $t, )*)
            }
            fn into_iter_pairs(self) -> Self::IntoIter {
                $pest_typed::chain!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, self, into_iter, [IGNORED; SKIP], $T0, $t0, $($T, $t, )*)
            }
        }
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> ::core::ops::Deref for $name<T0, $($T),*, SKIP, IGNORED> {
            type Target = ( ([IGNORED; SKIP], T0), $(([IGNORED; SKIP], $T), )* );
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> ::core::ops::DerefMut for $name<T0, $($T),*, SKIP, IGNORED> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
        impl<$T0: ::core::cmp::PartialEq, $($T: ::core::cmp::PartialEq),*, const SKIP: usize, IGNORED: ::core::cmp::PartialEq>
            ::core::cmp::PartialEq for $name<T0, $($T),*, SKIP, IGNORED>
        {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.content.$t0 == other.content.$t0
                $(
                    && self.content.$t == other.content.$t
                )*
            }
        }
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> $name<T0, $($T),*, SKIP, IGNORED> {
            /// Convert the reference of a sequence into a tuple of references of elements.
            pub fn as_ref(&self) -> ( &$T0, $(&$T, )* ) {
                ( &self.content.$t0.1, $(&self.content.$t.1, )* )
            }
        }
        impl<$T0, $($T),*, const SKIP: usize, IGNORED> ::core::convert::From<$name<T0, $($T),*, SKIP, IGNORED>>
            for ( T0, $($T, )* )
        {
            fn from(value: $name<T0, $($T),*, SKIP, IGNORED>) -> Self {
                ( value.content.$t0.1, $(value.content.$t.1, )* )
            }
        }
        impl<$T0: ::core::fmt::Debug, $($T: ::core::fmt::Debug),*, const SKIP: usize, IGNORED: ::core::fmt::Debug>
            ::core::fmt::Debug for $name<T0, $($T),*, SKIP, IGNORED>
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
