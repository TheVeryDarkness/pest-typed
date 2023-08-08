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
/// Implement pairs for tuples.
macro_rules! impl_tuples {
    ($name:ident, $pest_typed:ident, $T0:ident, $t0:tt, $( $T:ident, $t:tt, )* ) => {
        #[doc = "A sequence of several values."]
        #[derive(Clone, Debug)]
        pub struct $name<$T0, $($T, )*>($T0, $($T, )*);
        impl<
                'i: 'n,
                'n,
                R: $pest_typed::RuleType + 'n,
                $T0: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
                $($T: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>),*,
            > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<$T0, $($T, )*>
        {
            type Iter = $pest_typed::chains!($pest_typed, Iter, $T0, $($T, )*);
            type IntoIter = $pest_typed::chains!($pest_typed, IntoIter, $T0, $($T, )*);

            fn iter(&'n self) -> Self::Iter {
                $pest_typed::chain!($pest_typed, self, iter, $t0, $($t, )*)
            }
            fn into_iter(self) -> Self::IntoIter {
                $pest_typed::chain!($pest_typed, self, into_iter, $t0, $($t, )*)
            }
        }
        impl<$T0: PartialEq, $($T: PartialEq, )*> ::core::cmp::PartialEq for $name<T0, $($T, )*> {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.$t0 == other.$t0
                $(
                    && self.$t == other.$t
                )*
            }
        }
        impl<$T0, $($T, )*> ::core::convert::From<($T0, $($T, )*)> for $name<$T0, $($T, )*> {
            fn from(value: ($T0, $($T, )*)) -> Self {
                Self(value.$t0, $(value.$t, )*)
            }
        }
        impl<$T0, $($T, )*> $name<$T0, $($T, )*> {
            /// Convert the reference of a sequence into a tuple of references of elements.
            pub fn as_ref(&self) -> ( &$T0, $(&$T, )* ) {
                ( &self.$t0, $(&self.$t, )* )
            }
        }
    };
}

#[macro_export]
/// Generate sequences generics.
///
/// Also generate iterator type with [`crate::chain`] and [`crate::chain`].
macro_rules! seq {
    ($name:ident, $inner_type:ident, $pest_typed:ident, $T0:ident, $t0:tt, $( $T:ident, $t:tt, )* ) => {
        #[doc = "Match a sequence of several expressions."]
        #[derive(Clone, PartialEq)]
        pub struct $name<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > {
            content: $inner_type<T0, $($T, )*>,
            _phantom: ::core::marker::PhantomData<(&'i R, &'i IGNORED)>,
        }
        impl<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > ::core::convert::From<($T0, $($T, )*)> for $name<'i, R, $T0, $($T),*, IGNORED> {
            fn from(content: ($T0, $($T, )*)) -> Self {
                let content = $inner_type(content.$t0, $(content.$t, )*);
                Self { content, _phantom: ::core::marker::PhantomData }
            }
        }
        impl<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > $pest_typed::TypedNode<'i, R> for $name<'i, R, $T0, $($T),*, IGNORED> {
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
                        content
                    },
                    $(
                        {
                            let (next, _) = IGNORED::parse_with::<ATOMIC>(input, stack);
                            input = next;
                            let (next, content) = $T::try_parse_with::<ATOMIC>(input, stack, tracker)?;
                            input = next;
                            content
                        },
                    )*
                );

                Ok((input, Self::from(content)))
            }
            type Inner = $inner_type<T0, $($T, )*>;
            fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
                &node.content
            }
        }
        $pest_typed::impl_tuples!($inner_type, $pest_typed, $T0, $t0, $( $T, $t, )* );
        impl<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > ::core::ops::Deref for $name<'i, R, T0, $($T, )* IGNORED> {
            type Target = $inner_type<T0, $($T, )*>;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > ::core::ops::DerefMut for $name<'i, R, T0, $($T, )* IGNORED> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }
        impl<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > $pest_typed::Take for $name<'i, R, T0, $($T, )* IGNORED> {
            type Taken = $inner_type<T0, $($T, )*>;
            fn take(self) -> Self::Taken {
                self.content
            }
        }
        impl<
            'i: 'n,
            'n,
            R: $pest_typed::RuleType + 'n,
            $T0: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
            $($T: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>),*,
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<'i, R, T0, $($T, )* IGNORED> {
            type Iter = <$inner_type<T0, $($T, )*> as $pest_typed::iterators::Pairs<'i, 'n, R>>::Iter;
            type IntoIter = <$inner_type<T0, $($T, )*> as $pest_typed::iterators::Pairs<'i, 'n, R>>::IntoIter;

            fn iter(&'n self) -> Self::Iter {
                self.content.iter()
            }
            fn into_iter(self) -> Self::IntoIter {
                self.content.into_iter()
            }
        }
        impl<
            'i,
            R: $pest_typed::RuleType,
            $T0: $pest_typed::TypedNode<'i, R>,
            $($T: $pest_typed::TypedNode<'i, R>, )*
            IGNORED: $pest_typed::NeverFailedTypedNode<'i, R>,
        > ::core::fmt::Debug for $name<'i, R, T0, $($T),*, IGNORED> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple(::core::stringify!($name))
                    .field(&self.content.$t0)
                    $(.field(&self.content.$t))*
                    .finish()
            }
        }
    };
}

seq!(Seq2, Tuple2, crate, T0, 0, T1, 1,);
seq!(Seq3, Tuple3, crate, T0, 0, T1, 1, T2, 2,);
seq!(Seq4, Tuple4, crate, T0, 0, T1, 1, T2, 2, T3, 3,);
seq!(Seq5, Tuple5, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4,);
seq!(Seq6, Tuple6, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5,);
seq!(Seq7, Tuple7, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6,);
seq!(Seq8, Tuple8, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7,);
seq!(Seq9, Tuple9, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8,);
seq!(Seq10, Tuple10, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9,);
seq!(
    Seq11, Tuple11, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9,
    T10, 10,
);
seq!(
    Seq12, Tuple12, crate, T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9,
    T10, 10, T11, 11,
);
