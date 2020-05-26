macro_rules! repr {
    ($uxx:ty, #[$doc:meta] $name:ident { $(#[$attr:meta] $var:ident = $val:expr,)+ }) => {
        #[$doc]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $(#[$attr] $var = $val,)+
        }

        impl $name {
            pub(crate) fn _from(val: $uxx) -> Option<Self> {
                match val {
                    $($val => Some($name::$var),)+
                    _ => None,
                }
            }
        }
    }
}
