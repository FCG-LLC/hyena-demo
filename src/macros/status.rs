#[macro_export]
macro_rules! status {
    () => {{
        println!("");
        flush_stdout!()
    }};

    (ok) => {{
        let mut stdout = $crate::term::stdout().expect("failed to configure terminal");
        stdout.fg($crate::term::color::GREEN).expect("failed to set terminal color");
        println!(" OK");
        stdout.reset().expect("failed to configure terminal");
        flush_stdout!()
    }};

    (fail) => {{
        let mut stdout = $crate::term::stdout().expect("failed to configure terminal");
        stdout.fg($crate::term::color::RED).expect("failed to set terminal color");
        println!(" FAIL");
        stdout.reset().expect("failed to configure terminal");
        flush_stdout!()
    }};

    ($fmt: expr) => {
        status!($fmt,);
    };

    ($fmt: expr, $( $arg: tt )*) => {{
        let mut stdout = $crate::term::stdout().expect("failed to configure terminal");
        stdout.fg($crate::term::color::BRIGHT_YELLOW).expect("failed to set terminal color");
        print!("* ");
        stdout.fg($crate::term::color::WHITE).expect("failed to set terminal color");
        print!("{}", format!($fmt, $( $arg )*));
        stdout.reset().expect("failed to configure terminal");
        flush_stdout!()
    }};

    (+ $fmt: expr) => {
        status!(+ $fmt,);
    };

    (+ $fmt: expr, $( $arg: tt )*) => {{
        let mut stdout = $crate::term::stdout().expect("failed to configure terminal");
        stdout.fg($crate::term::color::WHITE).expect("failed to set terminal color");
        print!("{}", format!($fmt, $( $arg )*));
        stdout.reset().expect("failed to configure terminal");
        flush_stdout!()
    }};

}
