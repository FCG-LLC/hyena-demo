use std::time::Duration;
use num_traits::ToPrimitive;


pub fn stats_table(results: Vec<(&str, Duration, Option<usize>, Option<usize>)>)
{
    use prettytable::Table;
    use prettytable::row::Row;
    use prettytable::cell::Cell;
    use prettytable::format::{consts, Alignment};
    use term::{color, Attr};

    let mut table = Table::new();
    table.set_format(*consts::FORMAT_CLEAN);

    for &(name, value, count, bytes) in &results {
        let d = value;
        let secs = d.as_secs();
        let ns = d.subsec_nanos();

        let dd = secs.to_f64().unwrap() + { f64::from(ns) * 1e-9 };

        let mut row = vec![
            {
                let mut cell = Cell::new(name)
                    .with_style(Attr::ForegroundColor(color::GREEN))
                    .with_style(Attr::Bold);
                cell.align(Alignment::RIGHT);
                cell
            },
            {
                let mut cell = Cell::new({
                    &format!(
                        "{}.{:09} {}",
                        secs,
                        ns,
                        if secs <= 1 { "second" } else { "seconds" }
                    )
                }).with_style(Attr::ForegroundColor(color::BLUE));
                cell.align(Alignment::RIGHT);
                cell
            },
        ];

        let count_per_second = if let Some(count) = count {
            let per_second = {
                let conv = count.to_f64().unwrap();
                conv / dd
            };

            let mut cell = Cell::new({
                &format!("{:.2} rows/second", per_second)
            }).with_style(Attr::ForegroundColor(color::BLUE));
            cell.align(Alignment::RIGHT);
            row.push(cell);

            per_second
        } else {
            1_f64
        };

        if let Some(bytes) = bytes {
            let mut cell = Cell::new({
                let per_second = {
                    bytes.to_f64().unwrap() * count_per_second
                };

                &format!("{:.2} bytes/second", per_second)
            }).with_style(Attr::ForegroundColor(color::BLUE));
            cell.align(Alignment::RIGHT);
            row.push(cell);
        }

        table.add_row(Row::new(row));
    }

    table.printstd();
}
