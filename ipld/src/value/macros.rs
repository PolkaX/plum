// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

/// Construct a `ipld::IpldValue` from a JSON-like literal.
/// The macro is a modified version of `serde_json::json` macro, with extensions
/// for representing Ipld links and Ipld bytes.
#[macro_export(local_inner_macros)]
macro_rules! ipld {
    // Hide distracting implementation details from the generated rustdoc.
    ($($ipld:tt)+) => {
        ipld_internal!($($ipld)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! ipld_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an list [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: ipld_internal!(@list [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@list [$($elems:expr,)*]) => {
        ipld_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@list [$($elems:expr),*]) => {
        ipld_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@list [$($elems:expr,)*] null $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@list [$($elems:expr,)*] true $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@list [$($elems:expr,)*] false $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!(false)] $($rest)*)
    };

    // Next element is `bytes`.
    (@list [$($elems:expr,)*] bytes![$($bytes:tt)*] $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!(bytes![$($bytes)*])] $($rest)*)
    };

    // Next element is `link`.
    (@list [$($elems:expr,)*] link!($cid:expr) $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!(link!($cid))] $($rest)*)
    };

    // Next element is a list.
    (@list [$($elems:expr,)*] [$($list:tt)*] $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!([$($list)*])] $($rest)*)
    };

    // Next element is a map.
    (@list [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@list [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@list [$($elems:expr,)*] $last:expr) => {
        ipld_internal!(@list [$($elems,)* ipld_internal!($last)])
    };

    // Comma after the most recent element.
    (@list [$($elems:expr),*] , $($rest:tt)*) => {
        ipld_internal!(@list [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@list [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        ipld_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of a map {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: ipld_internal!(@map $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@map $map:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@map $map:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $map.insert(($($key)+).into(), $value);
        ipld_internal!(@map $map () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@map $map:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        ipld_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@map $map:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $map.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@map $map:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@map $map:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@map $map:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!(false)) $($rest)*);
    };

    // Next value is bytes.
    (@map $map:ident ($($key:tt)+) (: bytes![$($bytes:tt)*] $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!(bytes![$($bytes)*])) $($rest)*);
    };

    // Next value is link.
    (@map $map:ident ($($key:tt)+) (: link!($cid:expr) $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!(link!($cid))) $($rest)*);
    };

    // Next value is a list.
    (@map $map:ident ($($key:tt)+) (: [$($list:tt)*] $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!([$($list)*])) $($rest)*);
    };

    // Next value is a map.
    (@map $map:ident ($($key:tt)+) (: {$($map_:tt)*} $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!({$($map_)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@map $map:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@map $map:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        ipld_internal!(@map $map [$($key)+] (ipld_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@map $map:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        ipld_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error message.
    (@map $map:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        ipld_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@map $map:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        ipld_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@map $map:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        ipld_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@map $map:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@map $map:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        ipld_internal!(@map $map ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: ipld_internal!($($ipld)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::IpldValue::Null
    };

    (true) => {
        $crate::IpldValue::Bool(true)
    };

    (false) => {
        $crate::IpldValue::Bool(false)
    };

    (bytes![ $byte:expr; $n:expr ]) => {
        $crate::IpldValue::Bytes(ipld_internal_vec![$byte; $n].into())
    };
    (bytes![ $($byte:expr),* ]) => {
        $crate::IpldValue::Bytes(ipld_internal_vec![$($byte),*].into())
    };
    (bytes![ $($byte:expr,)* ]) => {
        $crate::IpldValue::Bytes(ipld_internal_vec![$($byte,)*].into())
    };

    (link!( $cid:expr )) => {
        $crate::IpldValue::Link($cid.parse().unwrap())
    };

    ([]) => {
        $crate::IpldValue::List(ipld_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::IpldValue::List(ipld_internal!(@list [] $($tt)+))
    };

    ({}) => {
        $crate::IpldValue::Map($crate::Map::new())
    };

    ({ $($tt:tt)+ }) => {
        $crate::IpldValue::Map({
            let mut map = $crate::Map::new();
            ipld_internal!(@map map () ($($tt)+) ($($tt)+));
            map
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::json_to_ipld(&$other).unwrap()
    };
}

// The ipld_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! ipld_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! ipld_unexpected {
    () => {};
}
