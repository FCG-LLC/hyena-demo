#[macro_export]
macro_rules! banner {
    (color $color: path, $( $arg: tt )* ) => {{
        banner!(@term).fg($color).expect("failed to set terminal color");
        banner!(@ $( $arg )*);
        banner!(clear);
    }};

    (red $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::RED, $( $arg )+);
    };

    (green $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::GREEN, $( $arg )+);
    };

    (blue $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::BLUE, $( $arg )+);
    };

    (yellow $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::YELLOW, $( $arg )+);
    };

    (magenta $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::MAGENTA, $( $arg )+);
    };

    (bright red $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::BRIGHT_RED, $( $arg )+);
    };

    (bright green $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::BRIGHT_GREEN, $( $arg )+);
    };

    (bright blue $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::BRIGHT_BLUE, $( $arg )+);
    };

    (bright yellow $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::BRIGHT_YELLOW, $( $arg )+);
    };

    (bright magenta $( $arg: tt ),+ $(,)*) => {
        banner!(color $crate::term::color::BRIGHT_MAGENTA, $( $arg )+);
    };

    (clear) => {
        banner!(@term).reset().expect("failed to set terminal color");
    };

    (@term) => {
        $crate::term::stdout().expect("failed to configure terminal")
    };

    (@ $( $arg: tt )*) => {{
        // 50 characters wide
        println!("");
        println!("{:=<79}+", "+");
        println!("| {: <77}|", format!($($arg)*));
        println!("{:=<79}+", "+");
        println!("");
    }};
}
