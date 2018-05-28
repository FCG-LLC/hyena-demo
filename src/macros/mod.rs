mod banner;
mod progress;
mod status;
mod time;

#[macro_export]
macro_rules! nsep {
    ($num: expr) => {{
        use $crate::pretty_toa::ThousandsSep;

        format!("{}", $num.thousands_sep())
    }};
}

#[macro_export]
macro_rules! flush_stdout {
    () => {{
        use std::io::{stdout, Write};
        stdout().flush().unwrap();
    }};
}
