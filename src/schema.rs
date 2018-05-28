use hyena_engine::Catalog;


#[inline]
pub fn display_columns(cat: &Catalog) -> Vec<(usize, String, String)> {
    let mut columns = cat.as_ref()
        .iter()
        .map(|(colid, col)| (*colid, col.to_string(), format!("{:?}", **col)))
        .collect::<Vec<_>>();

    columns.sort_by_key(|&(colid, _, _)| colid);

    columns
}

pub fn schema_table(cat: &Catalog) {
    use prettytable::Table;
    use prettytable::row::Row;
    use prettytable::cell::Cell;
    use prettytable::format::{consts, Alignment};
    use term::{color, Attr};

    let columns = display_columns(cat);

    let mut table = Table::new();
    table.set_format(*consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.set_titles(Row::new(vec![
        {
            let mut cell = Cell::new("ID")
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::Bold);
            cell.align(Alignment::CENTER);
            cell
        },
        {
            let mut cell = Cell::new("Name")
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::Bold);
            cell.align(Alignment::CENTER);
            cell
        },
        {
            let mut cell = Cell::new("Type")
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::Bold);
            cell.align(Alignment::CENTER);
            cell
        },
    ]));

    for &(colid, ref name, ref ty) in &columns {
        table.add_row(Row::new(vec![
            {
                let mut cell = Cell::new(&format!("{}", colid))
                    .with_style(Attr::ForegroundColor(color::GREEN));
                cell.align(Alignment::CENTER);
                cell
            },
            {
                let mut cell = Cell::new(name);
                cell.align(Alignment::LEFT);
                cell
            },
            {
                let mut cell = Cell::new(ty).with_style(Attr::ForegroundColor(color::BLUE));
                cell.align(Alignment::RIGHT);
                cell
            },
        ]));
    }

    table.printstd();
}
