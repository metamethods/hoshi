#[macro_export]
/// expects that the first argument to the macro is a struct that contains a locale method which returns an owned string
macro_rules! tl {
    ($struct_with_locale:expr, $($all:tt)*) => {
        t!($($all)*, locale = $struct_with_locale.locale().as_str())
    };
}
