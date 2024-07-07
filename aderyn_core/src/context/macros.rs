macro_rules! generate_capturable_methods {
    ($( $name:ident ),* $(,)*) => {

        #[derive(Clone)]
        pub enum Capturable {
            $($name($name),)*
        }

        $(
            impl From<$name> for Capturable {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }

            impl From<&$name> for Capturable {
                fn from(value: &$name) -> Self {
                    Self::$name(value.clone())
                }
            }
        )*
    };
}

pub(crate) use generate_capturable_methods;
