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
/// Generate choice helpers that can be used to traverse those branches.
/// Automatically called by [`crate::choices!`].
macro_rules! choices_helper {
    ($pest_typed:ident, $name:ident, $V0:ident, $v0:tt, $V1:ident, $v1:tt, $( $V:ident, $v:tt, )* ) => {
        /// Helper.
        pub enum $v0<Ret, $V0, $V1, $($V, )*> {
            /// Variant.
            $v0($V0),
            /// Variant.
            $v1($V1),
            $(
                /// Variant.
                $v($V),
            )*
            /// Wrapped result.
            Ok(Ret),
        }
        impl<Ret, $V0, $V1, $($V, )*> $pest_typed::predefined_node::NextChoice for $v0<Ret, $V0, $V1, $($V, )*> {
            type Next = $v1<Ret, $V1, $($V, )* >;
        }
        impl<Ret, $V0, $V1, $($V, )*> $v0<Ret, $V0, $V1, $($V, )* > {
            /// Invoke if this is the case.
            pub fn if_then(self, f: impl FnOnce($V0) -> Ret) -> $v1<Ret, $V1, $($V, )* > {
                match self {
                    Self::$v0(inner) => $v1::Ok(f(inner)),
                    Self::$v1(inner) => $v1::$v1(inner),
                    $(
                        Self::$v(inner) => $v1::$v(inner),
                    )*
                    Self::Ok(ret) => $v1::Ok(ret),
                }
            }
        }
        $crate::choices_helper!($pest_typed, $name, ($V0, $v0, $V1, $v1, $( $V, $v, )* ), $V1, $v1, $( $V, $v, )* );
    };
    ($pest_typed:ident, $name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, $V1:ident, $v1:tt, $( $V:ident, $v:tt, )* ) => {
        /// Helper.
        pub enum $v0<Ret, $V0, $V1, $($V, )*> {
            /// Variant.
            $v0($V0),
            /// Variant.
            $v1($V1),
            $(
                /// Variant.
                $v($V),
            )*
            /// Wrapped result.
            Ok(Ret),
        }
        impl<Ret, $V0, $V1, $($V, )*> $pest_typed::predefined_node::NextChoice for $v0<Ret, $V0, $V1, $($V, )*> {
            type Next = $v1<Ret, $V1, $($V, )* >;
        }
        impl<Ret, $V0, $V1, $($V, )*> $v0<Ret, $V0, $V1, $($V, )* > {
            /// If this is the case, invoke `f`, otherwise return next helper.
            pub fn else_if(self, f: impl FnOnce($V0) -> Ret) -> $v1<Ret, $V1, $($V, )* > {
                match self {
                    Self::$v0(res) => $v1::Ok(f(res)),
                    Self::$v1(inner) => $v1::$v1(inner),
                    $(
                        Self::$v(inner) => $v1::$v(inner),
                    )*
                    Self::Ok(ret) => $v1::Ok(ret),
                }
            }
        }
        $crate::choices_helper!($pest_typed, $name, ($_V0, $_v0, $( $_V, $_v, )* ), $V1, $v1, $( $V, $v, )* );
    };
    ($pest_typed:ident, $name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, ) => {
        /// Single helper for choices.
        pub enum $v0<Ret, $V0> {
            /// Variant.
            $v0($V0),
            /// Wrapped result.
            Ok(Ret),
        }
        impl<Ret, $V0> $v0<Ret, $V0> {
            /// If this is the case, invoke `f`, otherwise return wrapped value.
            pub fn else_then(self, f: impl FnOnce($V0) -> Ret) -> Ret {
                match self {
                    Self::$v0(res) => f(res),
                    Self::Ok(ret) => ret,
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
        /// Iterator type for choices.
        pub enum $iter_type<'i, 'n, R: $pest_typed::RuleType, $V0: $pest_typed::iterators::Pairs<'i, 'n, R>, $($V: $pest_typed::iterators::Pairs<'i, 'n, R>, )* > {
            /// Variant.
            $v0($V0::$iter_type, ::core::marker::PhantomData<(&'i R, &'n R)>),
            $(
                /// Variant.
                $v($V::$iter_type, ::core::marker::PhantomData<(&'i R, &'n R)>),
            )*
        }
        impl<'i: 'n, 'n, R: $pest_typed::RuleType + 'n, $V0: $pest_typed::iterators::Pairs<'i, 'n, R>, $($V: $pest_typed::iterators::Pairs<'i, 'n, R>, )* >
            ::core::iter::Iterator for $iter_type<'i, 'n, R, $V0, $($V, )*>
        {
            type Item = $item;
            fn next(&mut self) -> Option<<Self as ::core::iter::Iterator>::Item> {
                match self {
                    Self::$v0($v0, _) => $v0.next(),
                    $(
                        Self::$v($v, _) => $v.next(),
                    )*
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
    ($name:ident, $inner:ident, $pest_typed:ident, $helper:ident, $iter:ident, $V0:ident, $v0:tt, $( $V:ident, $v:tt, )* ) => {
        /// Match one of several expressions.
        #[derive(Clone, PartialEq)]
        pub struct $name<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* > {
            content: $inner<$V0, $($V, )*>,
            _phantom: ::core::marker::PhantomData<&'i R>,
        }
        impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* > $pest_typed::TypedNode<'i, R>
            for $name<'i, R, $V0, $($V, )* >
        {
            #[inline]
            fn try_parse_with<const ATOMIC: ::core::primitive::bool>(
                input: $pest_typed::Position<'i>,
                stack: &mut $pest_typed::Stack<$pest_typed::Span<'i>>,
                tracker: &mut $pest_typed::tracker::Tracker<'i, R>,
            ) -> ::core::result::Result<($pest_typed::Position<'i>, Self), ()> {
                if let Ok((input, res)) = $V0::try_parse_with::<ATOMIC>(input, stack, tracker) {
                    let content = $inner::$v0(res);
                    return Ok((input, Self { content, _phantom: ::core::marker::PhantomData }));
                }
                $(
                    if let Ok((input, res)) = $V::try_parse_with::<ATOMIC>(input, stack, tracker) {
                        let content = $inner::$v(res);
                        return Ok((input, Self { content, _phantom: ::core::marker::PhantomData }));
                    }
                )*
                Err(())
            }
            type Inner = $inner<$V0, $($V, )*>;
            fn deref_once<'n>(node: &'n Self) -> &'n Self::Inner {
                &node.content
            }
        }
        impl<
            'i: 'n,
            'n,
            R: $pest_typed::RuleType,
            $V0: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
            $($V: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>, )*
        > $pest_typed::iterators::Pairs<'i, 'n, R> for $name<'i, R, $V0, $($V, )* >
        {
            type Iter = <$inner<$V0, $($V, )*> as $pest_typed::iterators::Pairs<'i, 'n, R>>::Iter;
            type IntoIter = <$inner<$V0, $($V, )*> as $pest_typed::iterators::Pairs<'i, 'n, R>>::IntoIter;

            fn iter(&'n self) -> Self::Iter {
                self.content.iter()
            }
            fn into_iter(self) -> Self::IntoIter {
                self.content.into_iter()
            }
        }
        impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* >
            ::core::ops::Deref for $name<'i, R, $V0, $($V, )* >
        {
            type Target = $inner<$V0, $($V, )*>;
            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }
        impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* >
            $pest_typed::Take for $name<'i, R, $V0, $($V, )* >
        {
            type Taken = $inner<$V0, $($V, )*>;
            fn take(self) -> Self::Taken {
                self.content
            }
        }
        impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* >
            ::core::fmt::Debug for $name<'i, R, $V0, $($V, )* >
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(::core::stringify!($name)).field("content", &self.content).finish()
            }
        }
        #[derive(Clone, Debug, PartialEq)]
        /// Anonymous variant.
        pub enum $inner<$V0, $($V, )*> {
            /// Variant.
            $v0($V0),
            $(
                /// Variant.
                $v($V),
            )*
        }
        impl<$V0, $($V, )*> $inner<$V0, $($V, )*> {
            /// Start iteration.
            pub fn if_then<Ret>(&self, f: impl FnOnce(&$V0) -> Ret) -> <$helper::$v0<Ret, &$V0, $(&$V, )*> as $pest_typed::predefined_node::NextChoice>::Next {
                let starter = match self {
                    Self::$v0(res) => $helper::$v0::$v0(res),
                    $(Self::$v(inner) => $helper::$v0::$v(inner), )*
                };
                starter.if_then(f)
            }
            /// Access inner content.
            pub fn $v0(&self) -> ::core::option::Option<&$V0> {
                if let Self::$v0(res) = self {
                    Some(&res)
                } else {
                    None
                }
            }
            $(
                /// Access inner content.
                pub fn $v(&self) -> ::core::option::Option<&$V> {
                    if let Self::$v(res) = self {
                        Some(&res)
                    } else {
                        None
                    }
                }
            )*
        }
        impl<
            'i: 'n,
            'n,
            R: $pest_typed::RuleType + 'i,
            $V0: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>,
            $($V: $pest_typed::TypedNode<'i, R> + $pest_typed::iterators::Pairs<'i, 'n, R>, )*
        > $pest_typed::iterators::Pairs<'i, 'n, R> for $inner<$V0, $($V, )* >
        {
            type Iter = $iter::Iter<'i, 'n, R, $V0, $($V, )*>;
            type IntoIter = $iter::IntoIter<'i, 'n, R, $V0, $($V, )*>;

            fn iter(&'n self) -> Self::Iter {
                let phantom = ::core::marker::PhantomData;
                match self {
                    Self::$v0($v0) => Self::Iter::$v0($v0.iter(), phantom),
                    $(
                        Self::$v($v) => Self::Iter::$v($v.iter(), phantom),
                    )*
                }
            }
            fn into_iter(self) -> Self::IntoIter {
                let phantom = ::core::marker::PhantomData;
                match self {
                    Self::$v0($v0) => Self::IntoIter::$v0($v0.into_iter(), phantom),
                    $(
                        Self::$v($v) => Self::IntoIter::$v($v.into_iter(), phantom),
                    )*
                }
            }
        }
        /// For iterating choices.
        pub mod $helper {
            $crate::choices_helper!($pest_typed, $name, $V0, $v0, $( $V, $v, )*);
        }
        /// For iterating pairs.
        pub mod $iter {
            $crate::choices_iter!($name, $pest_typed, Iter, iter, &'n (dyn $pest_typed::iterators::Pair<'i, 'n, R>), $V0, $v0, $( $V, $v, )* );
            $crate::choices_iter!($name, $pest_typed, IntoIter, into_iter, $pest_typed::Box<dyn $pest_typed::iterators::Pair<'i, 'n, R> + 'n>, $V0, $v0, $( $V, $v, )* );
        }
    };
}

// Choices helper and iterator.

choices!(Choice2, Variant2, crate, ch2, it2, T0, _0, T1, _1,);
choices!(Choice3, Variant3, crate, ch3, it3, T0, _0, T1, _1, T2, _2,);
choices!(Choice4, Variant4, crate, ch4, it4, T0, _0, T1, _1, T2, _2, T3, _3,);
choices!(Choice5, Variant5, crate, ch5, it5, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4,);
choices!(Choice6, Variant6, crate, ch6, it6, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5,);
choices!(
    Choice7, Variant7, crate, ch7, it7, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,
);
choices!(
    Choice8, Variant8, crate, ch8, it8, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7,
    _7,
);
choices!(
    Choice9, Variant9, crate, ch9, it9, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7,
    _7, T8, _8,
);
choices!(
    Choice10, Variant10, crate, ch10, it10, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,
    T7, _7, T8, _8, T9, _9,
);
choices!(
    Choice11, Variant11, crate, ch11, it11, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,
    T7, _7, T8, _8, T9, _9, T10, _10,
);
choices!(
    Choice12, Variant12, crate, ch12, it12, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,
    T7, _7, T8, _8, T9, _9, T10, _10, T11, _11,
);
