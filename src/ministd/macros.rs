#[macro_export]
macro_rules! println {
    ( $( $x:expr ),* ) => {
        {
            $(
              crate::ministd::prelude::print($x);
            )*

              crate::ministd::prelude::print("\n");
        }
    };
}
