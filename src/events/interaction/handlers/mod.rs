pub mod application_command;

#[macro_export]
macro_rules! match_command_arm {
    ($a:literal) => {
        ($a, None, None)
    };

    ($a:literal, $b:literal) => {
        ($a, None, Some($b))
    };

    ($a:literal, $b:literal, $c:literal) => {
        ($a, Some($b), Some($c))
    };
}
