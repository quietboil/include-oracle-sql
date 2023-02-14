#[macro_export]
macro_rules! impl_sql {
    ( $sql_name:ident = $( { $kind:tt $name:ident ($($variant:tt $param:ident $ptype:tt)*) $doc:literal $s:tt $( $text:tt )+ } ),+ ) => {
        trait $sql_name {
            $( $crate::decl_method!{ $kind $name $doc () () () $($param $variant $ptype)* } )+
        }
        impl $sql_name for ::sibyl::Session<'_> {
            $( $crate::impl_method!{ $kind $name () () () ($($param $variant $ptype)*) => ($($variant $param)*) $($text)+ } )+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! decl_method {
    ( ? $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<'tr, 'st $(, $lt)* $(, $gen_type : ::sibyl::ToSql + Sync + Send + 'tr)*, F>(&'st self $($fn_params)* , row_cb: F)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<()>> + Send + 'tr>>
        where
            F: FnMut(::sibyl::Row) -> ::sibyl::Result<()>,
            F: Send, F: 'tr, Self: 'tr, 'st: 'tr $(, $lt : 'tr)*;
    };

    ( ! $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<'tr, 'st $(, $lt)* $(, $gen_type : ::sibyl::ToSql + Sync + Send + 'tr)*>(&'st self $($fn_params)*)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<usize>> + Send + 'tr>>
        where Self: 'tr, 'st: 'tr $(, $lt : 'tr)*;
    };

    ( . $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<'tr, 'st>(&'st self)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<::sibyl::Statement>> + Send + 'tr>>
        where Self: 'tr, 'st: 'tr;
    };

    ( $kind:tt $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : _ $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($lt)*)
            ($($gen_type)*)
            ($($fn_params)* , $param : impl sibyl::ToSql + Sync + Send + 'tr)
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : ($plt:lifetime & $ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($lt)* $plt)
            ($($gen_type)*)
            ($($fn_params)* , $param : & $plt $ptype)
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : ($ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($lt)*)
            ($($gen_type)*)
            ($($fn_params)* , $param : $ptype)
            $($tail)*
        }
    };

    ( $kind:tt $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident # [$alt:lifetime $gtype:ident] $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($lt)* $alt)
            ($($gen_type)* $gtype)
            ($($fn_params)* , $param : & $alt [ $gtype ] )
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:tt)*) ($($fn_params:tt)*) $param:ident # ($alt:lifetime $plt:lifetime & $ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($lt)* $alt $plt)
            ($($gen_type)*)
            ($($fn_params)* , $param : & $alt [ & $plt $ptype ] )
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($lt:lifetime)*) ($($gen_type:tt)*) ($($fn_params:tt)*) $param:ident # ($alt:lifetime $ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($lt)* $alt)
            ($($gen_type)*)
            ($($fn_params)* , $param : & $alt [ $ptype ] )
            $($tail)*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_method {
    ( ? $name:ident () () () () => () $text:literal ) => {
        fn $name<'tr, 'st, F>(&'st self, mut row_cb: F)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<()>> + Send + 'tr>>
        where
            F: FnMut(::sibyl::Row<'_>) -> ::sibyl::Result<()>,
            F: Send, F: 'tr, Self: 'tr, 'st: 'tr
        {
            ::std::boxed::Box::pin(async move {
                let stmt = self.prepare($text).await?;
                let rows = stmt.query(()).await?;
                while let Some(row) = rows.next().await? {
                    row_cb(row)?;
                }
                Ok(())
            })
        }
    };
    ( ? $name:ident ($($lt:lifetime)*) () ($($fn_params:tt)+) () => ($(: $arg:ident)+) $($text:tt)+) => {
        fn $name<'tr, 'st $(, $lt)*, F>(&'st self $($fn_params)+, mut row_cb: F)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<()>> + Send + 'tr>>
        where
            F: FnMut(::sibyl::Row) -> ::sibyl::Result<()>,
            F: Send, F: 'tr, Self: 'tr, 'st: 'tr $(, $lt : 'tr)*
        {
            ::std::boxed::Box::pin(async move {
                let stmt = self.prepare( $crate::sql_literal!( $($text)+ ) ).await?;
                let rows = stmt.query( ::include_oracle_sql_args::map!( $($arg)+ => $($text)+ ) ).await?;
                while let Some(row) = rows.next().await? {
                    row_cb(row)?;
                }
                Ok(())
            })
        }
    };
    ( ? $name:ident ($($lt:lifetime)*) ($($gen_type:tt)*) ($($fn_params:tt)+) () => ($($pv:tt $arg:ident)+) $($text:tt)+) => {
        fn $name<'tr, 'st $(, $lt)* $(, $gen_type : ::sibyl::ToSql + Sync + Send + 'tr)*, F>(&'st self $($fn_params)+, mut row_cb: F)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<()>> + Send + 'tr>>
        where
            F: FnMut(::sibyl::Row<'_>) -> ::sibyl::Result<()>,
            F: Send, F: 'tr, Self: 'tr, 'st: 'tr $(, $lt : 'tr)*
        {
            ::std::boxed::Box::pin(async move {
                let mut stmt = ::std::string::String::with_capacity( $crate::sql_len!($($text)+) );
                let mut i = 0;
                $crate::dynamic_sql!(stmt i $($text)+);
                let stmt = self.prepare(&stmt).await?;
                let rows = stmt.query( ::include_oracle_sql_args::map!( $($arg)+ => $($text)+ ) ).await?;
                while let Some(row) = rows.next().await? {
                    row_cb(row)?;
                }
                Ok(())
            })
        }
    };

    ( ! $name:ident () () () () => () $text:literal ) => {
        fn $name<'tr, 'st>(&'st self)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<usize>> + Send + 'tr>>
        where Self: 'tr, 'st: 'tr
        {
            ::std::boxed::Box::pin(async move {
                let stmt = self.prepare($text).await?;
                stmt.execute(()).await
            })
        }
    };
    ( ! $name:ident ($($lt:lifetime)*) () ($($fn_params:tt)+) () => ((: $arg:ident)+) $($text:tt)+) => {
        fn $name<'tr, 'st $(, $lt)*>(&'st self $($fn_params)+)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<usize>> + Send + 'tr>>
        where Self: 'tr, 'st: 'tr $(, $lt : 'tr)*
        {
            ::std::boxed::Box::pin(async move {
                let stmt = self.prepare( $crate::sql_literal!( $($text)+ ) ).await?;
                stmt.execute( ::include_oracle_sql_args::map!( $($arg)+ => $($text)+ ) ).await
            })
        }
    };
    ( ! $name:ident ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)+) () => ($($pv:tt $arg:ident)+) $($text:tt)+) => {
        fn $name<'tr, 'st $(, $lt)* $(, $gen_type : ::sibyl::ToSql + Sync + Send + 'tr)*>(&'st self $($fn_params)+)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<usize>> + Send + 'tr>>
        where Self: 'tr, 'st: 'tr $(, $lt : 'tr)*
        {
            ::std::boxed::Box::pin(async move {
                let mut stmt = ::std::string::String::with_capacity( $crate::sql_len!($($text)+) );
                let mut i = 0;
                $crate::dynamic_sql!(stmt i $($text)+);
                let stmt = self.prepare(&stmt).await?;
                let args = ::include_oracle_sql_args::map!( $($arg)+ => $($text)+ );
                stmt.execute( args ).await
            })
        }
    };

    ( . $name:ident ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) () => ($($pv:tt $arg:ident)*) $($text:tt)+) => {
        fn $name<'tr, 'st>(&'st self)
        -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::sibyl::Result<::sibyl::Statement>> + Send + 'tr>>
        where Self: 'tr, 'st: 'tr
        {
            ::std::boxed::Box::pin(async move {
                self.prepare( $crate::sql_literal!( $($text)+ ) ).await
            })
        }
    };

    ( $kind:tt $name:ident ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : _ $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($lt)*)
            ($($gen_type)*)
            ($($fn_params)* , $param : impl ::sibyl::ToSql + Sync + Send + 'tr)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : ($plt:lifetime & $ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($lt)* $plt)
            ($($gen_type)*)
            ($($fn_params)* , $param : & $plt $ptype)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : ($ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($lt)*)
            ($($gen_type)*)
            ($($fn_params)* , $param : $ptype)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($lt:lifetime)*) ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # [$alt:lifetime $gtype:ident] $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($lt)* $alt)
            ($($gen_type)* $gtype)
            ($($fn_params)* , $param : & $alt [ $gtype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($lt:lifetime)*)  ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # ($alt:lifetime $plt:lifetime & $ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($lt)* $alt $plt)
            ($($gen_type)*)
            ($($fn_params)* , $param : & $alt [ & $plt $ptype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($lt:lifetime)*)  ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # ($alt:lifetime $ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($lt)* $alt)
            ($($gen_type)*)
            ($($fn_params)* , $param : & $alt [ $ptype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
}
