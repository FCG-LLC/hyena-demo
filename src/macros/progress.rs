#[macro_export]
macro_rules! progress {
    ($steps: expr) => {{
        use $crate::indicatif::{ProgressBar, ProgressStyle};

        let bar = ProgressBar::new($steps as u64);

        bar.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:80.yellow/dimyellow} {pos:>7}/{len:7} {msg}"));
//             .progress_chars("##-"));
        bar
    }};

    (advance $bar: expr, $msg: expr) => {
        $bar.inc(1);
        $bar.set_message($msg);
    };

    (advance $bar: expr) => {
        $bar.inc(1);
    };

    (finish message $bar: expr, $msg: expr) => {
        $bar.finish_with_message($msg);
    };

    (finish $bar: expr) => {
        $bar.finish_and_clear();
    };
}
