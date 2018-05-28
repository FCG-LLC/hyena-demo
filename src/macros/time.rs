#[macro_export]
macro_rules! time {
    ($results: expr, $name: expr, $what: block) => {
        time!(@ $results, None, None, $name, $what)
    };

    ($results: expr, $count: expr, $name: expr, $what: block) => {
        time!(@ $results, Some($count), None, $name, $what)
    };

    ($results: expr, $count: expr, $bytes: expr, $name: expr, $what: block) => {{
        time!(@ $results, Some($count), Some($bytes), $name, $what)
    }};

    (@ $results: expr, $count: expr, $bytes: expr, $name: expr, $what: block) => {{
        let t = time!();
        let res = $what;
        let elapsed = t.elapsed();
        $results.push(($name, elapsed, $count, $bytes));
        res
    }};

    (print $name: expr, $what: block) => {{
        let t = time!();
        let res = $what;
        time!(res t, $name);
        res
    }};

    (res $t: expr, $name: expr) => {{
        let d = $t.elapsed();
        let secs = d.as_secs();
        let ns = d.subsec_nanos();

        println!("{}: {}.{:09} {}", $name, secs, ns, if secs <= 1 { "second" } else { "seconds" });
    }};

    (delta $instant: expr) => {{
        use std::time::Instant;

        Instant::now() - $instant
    }};

    () => {{
        use std::time::Instant;

        Instant::now()
    }};
}
