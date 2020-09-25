#[macro_export]
#[allow_internal_unstable(box_syntax)]
macro_rules! string_data {
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(box [$($x.to_string()),+])
    );
}