#[macro_export]
macro_rules! impl_sql {
    ( $sql_name:ident = $( { $kind:tt $name:ident ($($variant:tt $param:ident $ptype:tt)*) $doc:literal $s:tt $( $text:tt )+ } ),+ ) => {
        trait $sql_name {
            $( $crate::decl_method!{ $kind $name $doc () () $($param $variant $ptype)* } )+
        }
        impl $sql_name for ::sibyl::Session<'_> {
            $( $crate::impl_method!{ $kind $name () () ($($param $variant $ptype)*) => ($($variant $param)*) $($text)+ } )+
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! decl_method {
    ( ? $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<$($gen_type : ::sibyl::ToSql ,)* F>(&self $($fn_params)* , row_cb: F) -> ::sibyl::Result<()>
        where F: FnMut(::sibyl::Row) -> ::sibyl::Result<()>;
    };

    ( ! $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name<$($gen_type : ::sibyl::ToSql),*>(&self $($fn_params)*) -> ::sibyl::Result<usize>;
    };

    ( . $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) ) => {
        #[doc=$doc]
        fn $name(&self) -> ::sibyl::Result<::sibyl::Statement>;
    };

    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : _ $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)*)
            ($($fn_params)* , $param : impl ::sibyl::ToSql)
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident : ($ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)*)
            ($($fn_params)* , $param : $ptype)
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident # [$gtype:ident] $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)* $gtype)
            ($($fn_params)* , $param : & [ $gtype ] )
            $($tail)*
        }
    };
    ( $kind:tt $name:ident $doc:literal ($($gen_type:ident)*) ($($fn_params:tt)*) $param:ident # ($ptype:ty) $($tail:tt)* ) => {
        $crate::decl_method!{
            $kind
            $name
            $doc
            ($($gen_type)*)
            ($($fn_params)* , $param : & [ $ptype ] )
            $($tail)*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_method {
    ( ? $name:ident () () () => () $text:literal ) => {
        fn $name<F>(&self, mut row_cb: F) -> ::sibyl::Result<()>
        where F: FnMut(::sibyl::Row) -> ::sibyl::Result<()>
        {
            let stmt = self.prepare($text)?;
            let rows = stmt.query(())?;
            while let Some(row) = rows.next()? {
                row_cb(row)?;
            }
            Ok(())
        }
    };
    ( ? $name:ident () ($($fn_params:tt)+) () => ($(: $arg:ident)+) $($text:tt)+) => {
        fn $name<F>(&self $($fn_params)+ , mut row_cb: F) -> ::sibyl::Result<()>
        where F: FnMut(::sibyl::Row) -> ::sibyl::Result<()>
        {
            let stmt = self.prepare( $crate::sql_literal!( $($text)+ ) )?;
            let rows = stmt.query( $crate::util::map!( $($arg)+ => $($text)+ ) )?;
            while let Some(row) = rows.next()? {
                row_cb(row)?;
            }
            Ok(())
        }
    };
    ( ? $name:ident ($($gen_type:ident)*) ($($fn_params:tt)+) () => ($($pv:tt $arg:ident)+) $($text:tt)+) => {
        fn $name<$($gen_type : ::sibyl::ToSql ,)* F>(&self $($fn_params)+, mut row_cb: F) -> ::sibyl::Result<()>
        where F: FnMut(::sibyl::Row) -> ::sibyl::Result<()>
        {
            let mut stmt = ::std::string::String::with_capacity( $crate::sql_len!($($text)+) );
            let mut i = 0;
            $crate::dynamic_sql!(stmt i $($text)+);
            let stmt = self.prepare(&stmt)?;
            let rows = stmt.query( $crate::util::map!( $($arg)+ => $($text)+ ) )?;
            while let Some(row) = rows.next()? {
                row_cb(row)?;
            }
            Ok(())
        }
    };

    ( ! $name:ident () () () => () $text:literal ) => {
        fn $name(&self) -> ::sibyl::Result<usize> {
            let stmt = self.prepare($text)?;
            stmt.execute(())
        }
    };
    ( ! $name:ident () ($($fn_params:tt)+) () => ($(: $arg:ident)+) $($text:tt)+) => {
        fn $name(&self $($fn_params)+) -> ::sibyl::Result<usize> {
            let stmt = self.prepare( $crate::sql_literal!( $($text)+ ) )?;
            stmt.execute( $crate::util::map!( $($arg)+ => $($text)+ ) )
        }
    };
    ( ! $name:ident ($($gen_type:ident)*) ($($fn_params:tt)+) () => ($($pv:tt $arg:ident)+) $($text:tt)+) => {
        fn $name<$($gen_type : ::sibyl::ToSql),*>(&self $($fn_params)+ ) -> ::sibyl::Result<usize> {
            let mut stmt = ::std::string::String::with_capacity( $crate::sql_len!($($text)+) );
            let mut i = 0;
            $crate::dynamic_sql!(stmt i $($text)+);
            let stmt = self.prepare(&stmt)?;
            stmt.execute( $crate::util::map!( $($arg)+ => $($text)+ ) )
        }
    };

    ( . $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) () => ($($pv:tt $arg:ident)*) $($text:tt)+) => {
        fn $name(&self) -> ::sibyl::Result<::sibyl::Statement> {
            self.prepare( $crate::sql_literal!( $($text)+ ) )
        }
    };

    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : _ $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)*)
            ($($fn_params)* , $param : impl ::sibyl::ToSql)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident : ($ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)*)
            ($($fn_params)* , $param : $ptype)
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # [$gtype:ident] $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)* $gtype)
            ($($fn_params)* , $param : & [ $gtype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
    ( $kind:tt $name:ident ($($gen_type:ident)*) ($($fn_params:tt)*) ($param:ident # ($ptype:ty) $($tail:tt)*) => ($($pv:tt $param_name:ident)+) $($text:tt)+)  => {
        $crate::impl_method!{
            $kind
            $name
            ($($gen_type)*)
            ($($fn_params)* , $param : & [ $ptype ])
            ($($tail)*)
            =>
            ($($pv $param_name)+)
            $($text)+
        }
    };
}
