pub mod validators;

#[macro_export]
macro_rules! update_if_changed {
    ($source:expr, $param:expr) => {{
        if $source != $param {
            $source = $param
        }
    }};
}
