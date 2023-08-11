// pest-typed. A statically typed version of pest.
// Copyright (c) 2023 黄博奕
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Definition of choices-related macros and types.

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
        /// - `'i`: The lifetime of input string.
        /// - `'n`: The lifetime of the root node.
        /// - `Ret`: Return value type.
        /// - `R`: Rule type.
        /// - `...`: Choices branches.
        pub struct $v0<'i, 'n, Ret, R: $pest_typed::RuleType, $_V0: $pest_typed::TypedNode<'i, R>, $($_V: $pest_typed::TypedNode<'i, R>, )*> {
            pub(super) result: ::core::result::Result<Ret, &'n super::$name<'i, R, $_V0, $($_V, )*>>,
        }

        impl<'i: 'n, 'n, Ret, R: $pest_typed::RuleType, $_V0: $pest_typed::TypedNode<'i, R>, $($_V: $pest_typed::TypedNode<'i, R>, )*>
            $pest_typed::choices::NextChoice for $v0<'i, 'n, Ret, R, $_V0, $($_V, )* > {
            type Next = $v1<'i, 'n, Ret, R, $_V0, $($_V, )* >;
        }
        impl<'i: 'n, 'n, Ret, R: $pest_typed::RuleType, $_V0: $pest_typed::TypedNode<'i, R>, $($_V: $pest_typed::TypedNode<'i, R>, )*>
            $v0<'i, 'n, Ret, R, $_V0, $($_V, )* >
        {
            pub fn else_if(self, f: impl FnOnce(&$V0) -> Ret) -> <Self as $pest_typed::choices::NextChoice>::Next {
                match self.result {
                    Err(super::$name::$v0(matched, _)) => {
                        let result: Ret = f(matched);
                        $v1 { result: Ok(result) }
                    }
                    Err(choices) => $v1 { result: Err(choices) },
                    Ok(ret) => $v1 { result: Ok(ret) },
                }
            }
        }
        $crate::choices_helper!($pest_typed, $name, ($_V0, $_v0, $( $_V, $_v, )* ), $V1, $v1, $( $V, $v, )* );
    };
    ($pest_typed:ident, $name:ident, ($_V0:ident, $_v0:tt, $( $_V:ident, $_v:tt, )* ), $V0:ident, $v0:tt, ) => {
        pub struct $v0<'i, 'n, Ret, R: $pest_typed::RuleType, $_V0: $pest_typed::TypedNode<'i, R>, $($_V: $pest_typed::TypedNode<'i, R>, )*> {
            pub(super) result: ::core::result::Result<Ret, &'n super::$name<'i, R, $_V0, $($_V, )*>>,
        }
        impl<'i: 'n, 'n, Ret, R: $pest_typed::RuleType, $_V0: $pest_typed::TypedNode<'i, R>, $($_V: $pest_typed::TypedNode<'i, R>, )*>
            $v0<'i, 'n, Ret, R, $_V0, $($_V, )* >
        {
            pub fn else_then(self, f: impl FnOnce(&'n $V0) -> Ret) -> Ret {
                match self.result {
                    Err(super::$name::$v0(matched, _)) => f(matched),
                    Err(_) => ::core::unreachable!(),
                    Ok(ret) => ret,
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
            $v0($V0::$iter_type, ::core::marker::PhantomData<(&'i R, &'n R)>),
            $($v($V::$iter_type, ::core::marker::PhantomData<(&'i R, &'n R)>), )*
        }
        impl<'i: 'n, 'n, R: $pest_typed::RuleType + 'n, $V0: $pest_typed::iterators::Pairs<'i, 'n, R>, $($V: $pest_typed::iterators::Pairs<'i, 'n, R>, )* >
            ::core::iter::Iterator for $iter_type<'i, 'n, R, $V0, $($V, )*>
        {
            type Item = $item;
            fn next(&mut self) -> Option<<Self as ::core::iter::Iterator>::Item> {
                match self {
                    Self::$v0($v0, _) => $v0.next(),
                    $(Self::$v($v, _) => $v.next(), )*
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
    ($name:ident, $pest_typed:ident, $helper:ident, $iter:ident, $V0:ident, $v0:tt, $( $V:ident, $v:tt, )* ) => {
        /// Match either of two expressions.
        #[derive(Clone, PartialEq)]
        pub enum $name<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* > {
            #[doc = ::core::stringify!(Variant $v0 for choice $V0.)]
            $v0($V0, ::core::marker::PhantomData<&'i R>),
            $(
                #[doc = ::core::stringify!(Variant $v for choice $V.)]
                $v($V, ::core::marker::PhantomData<&'i R>),
            )*
        }
        impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* > $name<'i, R, $V0, $($V, )* > {
            /// Invoke if is not None and is the first case.
            pub fn if_then<'n, Ret>(&'n self, f: impl FnOnce(&$V0) -> Ret) -> <$helper::$v0<'i, 'n, Ret, R, $V0, $($V, )* > as $crate::choices::NextChoice>::Next {
                let helper = $helper::$v0 { result: Err(self) };
                helper.else_if(f)
            }
            /// Access inner node if matched.
            pub fn $v0(&self) -> ::core::option::Option<&$V0> {
                if let Self::$v0(res, _) = self {
                    ::core::option::Option::Some(res)
                } else {
                    ::core::option::Option::None
                }
            }
            $(
                /// Access inner node if matched.
                pub fn $v(&self) -> ::core::option::Option<&$V> {
                    if let Self::$v(res, _) = self {
                        ::core::option::Option::Some(res)
                    } else {
                        ::core::option::Option::None
                    }
                }
            )*
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
                let res = $pest_typed::predefined_node::restore_on_err(stack, |stack| $V0::try_parse_with::<ATOMIC>(input, stack, tracker));
                if let Ok((input, res)) = res {
                    return Ok((input, Self::$v0(res, ::core::marker::PhantomData)));
                }
                $(
                    let res = $pest_typed::predefined_node::restore_on_err(stack, |stack| $V::try_parse_with::<ATOMIC>(input, stack, tracker));
                    if let Ok((input, res)) = res {
                        return Ok((input, Self::$v(res, ::core::marker::PhantomData)));
                    }
                )*
                Err(())
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
            type Iter = $iter::Iter<'i, 'n, R, $V0, $($V, )*>;
            type IntoIter = $iter::IntoIter<'i, 'n, R, $V0, $($V, )*>;

            fn iter(&'n self) -> Self::Iter {
                match self {
                    Self::$v0($v0, _) => Self::Iter::$v0($v0.iter(), ::core::marker::PhantomData),
                    $(Self::$v($v, _) => Self::Iter::$v($v.iter(), ::core::marker::PhantomData), )*
                }
            }
            fn into_iter(self) -> Self::IntoIter {
                match self {
                    Self::$v0($v0, _) => Self::IntoIter::$v0($v0.into_iter(), ::core::marker::PhantomData),
                    $(Self::$v($v, _) => Self::IntoIter::$v($v.into_iter(), ::core::marker::PhantomData), )*
                }
            }
        }
        impl<'i, R: $pest_typed::RuleType, $V0: $pest_typed::TypedNode<'i, R>, $($V: $pest_typed::TypedNode<'i, R>, )* >
            ::core::fmt::Debug for $name<'i, R, $V0, $($V, )* >
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    Self::$v0($v0, _) => f.debug_struct(::core::stringify!($name)).field(&::core::stringify!($v0), &$v0).finish(),
                    $(
                        Self::$v($v, _) => f.debug_struct(::core::stringify!($name)).field(&::core::stringify!($v), &$v).finish(),
                    )*
                }
            }
        }
        mod $helper {
            $crate::choices_helper!($pest_typed, $name, ($V0, $v0, $( $V, $v, )* ), $V0, $v0, $( $V, $v, )* );
        }
        mod $iter {
            $crate::choices_iter!($name, $pest_typed, Iter, iter, &'n (dyn $pest_typed::iterators::Pair<'i, 'n, R>), $V0, $v0, $( $V, $v, )* );
            $crate::choices_iter!($name, $pest_typed, IntoIter, into_iter, $pest_typed::Box<dyn $pest_typed::iterators::Pair<'i, 'n, R> + 'n>, $V0, $v0, $( $V, $v, )* );
        }
    };
}

// Choices helper and iterator.

choices!(Choice2, crate, ch2, it2, T0, _0, T1, _1,);
choices!(Choice3, crate, ch3, it3, T0, _0, T1, _1, T2, _2,);
choices!(Choice4, crate, ch4, it4, T0, _0, T1, _1, T2, _2, T3, _3,);
choices!(Choice5, crate, ch5, it5, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4,);
choices!(Choice6, crate, ch6, it6, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5,);
choices!(Choice7, crate, ch7, it7, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6,);
choices!(Choice8, crate, ch8, it8, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,);
choices!(
    Choice9, crate, ch9, it9, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7, T8,
    _8,
);
choices!(
    Choice10, crate, ch10, it10, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
    T8, _8, T9, _9,
);
choices!(
    Choice11, crate, ch11, it11, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
    T8, _8, T9, _9, T10, _10,
);
choices!(
    Choice12, crate, ch12, it12, T0, _0, T1, _1, T2, _2, T3, _3, T4, _4, T5, _5, T6, _6, T7, _7,
    T8, _8, T9, _9, T10, _10, T11, _11,
);
