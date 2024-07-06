macro_rules! generate_capture_methods {
    ($( $name:ident ),* $(,)*) => {
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

pub(crate) use generate_capture_methods;
