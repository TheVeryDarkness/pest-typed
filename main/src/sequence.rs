// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Definition of choices-related macros and types.

#[macro_export]
/// Generate sequences generics.
///
/// Also generate iterator type with [`crate::chain`] and [`crate::chain`].
macro_rules! seq {
    ($name:ident, $pest_typed:ident, $T0:ident, $t0:tt, $( $T:ident, $t:tt, )* ) => {
        #[doc = "Match a sequence of several expressions."]
        #[derive(Clone)]
        pub struct $name<$T0, $($T),*, IGNORED, > {
            #[doc = "Matched and skipped expressions."]
            pub content: ( (IGNORED, $T0), $((IGNORED, $T), )* ),
        }
        impl<$T0, $($T, )* IGNORED> ::core::convert::From<( (IGNORED, $T0), $((IGNORED, $T), )* )>
            for $name<$T0, $($T),*, IGNORED>
        {
            fn from(content: ( (IGNORED, $T0), $((IGNORED, $T), )* )) -> Self {
                Self { content }
            }
        }
        impl<
                'i,
                R: $pest_typed::RuleType,
                $T0: $pest_typed::TypedNode<'i, R>,
                $($T: $pest_typed::TypedNode<'i, R>, )*
                IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
            > $pest_typed::TypedNode<'i, R> for $name<$T0, $($T),*, IGNORED>
        {
            #[inline]
            fn try_parse_with<const ATOMIC: bool>(
                mut input: $pest_typed::Position<'i>,
                stack: &mut $pest_typed::Stack<$pest_typed::Span<'i>>,
                tracker: &mut $pest_typed::tracker::Tracker<'i, R>,
            ) -> ::core::result::Result<($pest_typed::Position<'i>, Self), ()> {
                let content =
                (
                    {
                        let (next, content) = T0::try_parse_with::<ATOMIC>(input, stack, tracker)?;
                        input = next;
                        (IGNORED::default(), content)
                    },
                    $(
                        {
                            let (next, skipped) = IGNORED::parse_with::<ATOMIC>(input, stack);
                            input = next;
                            let (next, content) = $T::try_parse_with::<ATOMIC>(input, stack, tracker)?;
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
                IGNORED: $pest_typed::NeverFailedTypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
            > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<$T0, $($T),*, IGNORED>
        {
            type Iter = $pest_typed::chains!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, Iter, (IGNORED, $T0), $((IGNORED, $T), )*);
            type IntoIter = $pest_typed::chains!($pest_typed, $pest_typed::iterators::Pairs<'i, 'n, R>, IntoIter, (IGNORED, $T0), $((IGNORED, $T), )*);

            fn iter(&'n self) -> Self::Iter {
                $pest_typed::chain!($pest_typed, self, iter, $t0, $($t, )*)
            }
            fn into_iter(self) -> Self::IntoIter {
                $pest_typed::chain!($pest_typed, self, into_iter, $t0, $($t, )*)
            }
        }
        impl<$T0, $($T),*, IGNORED> ::core::ops::Deref for $name<T0, $($T),*, IGNORED> {
            type Target = ( (IGNORED, T0), $((IGNORED, $T), )* );
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<$T0, $($T),*, IGNORED> ::core::ops::DerefMut for $name<T0, $($T),*, IGNORED> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
        impl<$T0: ::core::cmp::PartialEq, $($T: ::core::cmp::PartialEq),*, IGNORED: ::core::cmp::PartialEq>
            ::core::cmp::PartialEq for $name<T0, $($T),*, IGNORED>
        {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.content.$t0 == other.content.$t0
                $(
                    && self.content.$t == other.content.$t
                )*
            }
        }
        impl<$T0, $($T),*, IGNORED> $name<T0, $($T),*, IGNORED>
        {
            /// Convert the reference of a sequence into a tuple of references of elements.
            pub fn as_ref(&self) -> ( &$T0, $(&$T, )* ) {
                ( &self.content.$t0.1, $(&self.content.$t.1, )* )
            }
        }
        impl<$T0, $($T),*, IGNORED> ::core::convert::From<$name<T0, $($T),*, IGNORED>>
            for ( T0, $($T, )* )
        {
            fn from(value: $name<T0, $($T),*, IGNORED>) -> Self {
                ( value.content.$t0.1, $(value.content.$t.1, )* )
            }
        }
        impl<$T0: ::core::fmt::Debug, $($T: ::core::fmt::Debug),*, IGNORED: ::core::fmt::Debug>
            ::core::fmt::Debug for $name<T0, $($T),*, IGNORED>
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("Seq")
                    .field(&self.content.$t0)
                    $(.field(&self.content.$t))*
                    .finish()
            }
        }
    };
}

seq!(Seq2, crate, T0, 0, T1, 1,);
seq!(Seq3, crate, T0, 0, T1, 1, T2, 2,);
seq!(Seq4, crate, T0, 0, T1, 1, T2, 2, T3, 3,);
seq!(Seq5, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4,);
seq!(Seq6, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5,);
seq!(Seq7, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6,);
seq!(Seq8, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7,);
seq!(Seq9, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8,);
seq!(Seq10, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9,);
seq!(Seq11, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10,);
seq!(
    Seq12, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10,
    T11, 11,
);
