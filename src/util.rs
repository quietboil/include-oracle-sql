pub use ::include_oracle_sql_args::map;

#[macro_export]
#[doc(hidden)]
macro_rules! sql_literal {
    ($text:literal) => {
        $text
    };
    ($text:literal : $param:ident) => {
        ::std::concat!( $text, ':', ::std::stringify!($param) )
    };
    ($text:literal : $param:ident $($tail:tt)+) => {
        std::concat!(
            $text, ':', ::std::stringify!($param),
            $crate::sql_literal!( $($tail)+ )
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! sql_len {
    () => { 0 };
    ($text:literal $($tail:tt)*) => { $text.len() + $crate::sql_len!($($tail)*) };
    (: $head:ident $($tail:tt)*) => { ::std::stringify!($head).len() + 1 + $crate::sql_len!($($tail)*) };
    (# $head:ident $($tail:tt)*) => { ::std::stringify!($head).len() + 1 + ($head.len() - 1) * 5 + $crate::sql_len!($($tail)*) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! dynamic_sql {
    ($stmt:ident $i:ident) => {};
    ($stmt:ident $i:ident $text:literal $($tail:tt)*) => {
        $stmt.push_str($text);
        $crate::dynamic_sql!($stmt $i $($tail)*);
    };
    ($stmt:ident $i:ident : $param:ident $($tail:tt)*) => {
        $i += 1;
        $stmt.push_str(&::std::format!(":{}", ::std::stringify!($param)));
        $crate::dynamic_sql!($stmt $i $($tail)*);
    };
    ($stmt:ident $i:ident # $param:ident $($tail:tt)*) => {
        if $param.len() == 0 {
            $stmt.push_str("NULL");
        } else {
            $i += 1;
            $stmt.push_str(&::std::format!(":{}", ::std::stringify!($param)));
            for _ in 1 .. $param.len() {
                $i += 1;
                $stmt.push_str(&::std::format!(", :{}", $i));
            }
        }
        $crate::dynamic_sql!($stmt $i $($tail)*);
    };
}
