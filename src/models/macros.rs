#[macro_export]
macro_rules! to_sql {

    (pub struct $name:ident { $($fname:ident: $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
            fn field_types() -> &'static [&'static str]{
                static TYPES: &'static [&'static str] = &[$(stringify!($ftype)),*];
                TYPES
            }
        }
    }
}
