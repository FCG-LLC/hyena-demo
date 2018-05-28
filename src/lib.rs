extern crate failure;
extern crate hyena_engine;
extern crate hyena_common;
extern crate prettytable;
extern crate syntect;
extern crate num_traits;

pub extern crate indicatif;
pub extern crate term;
pub extern crate pretty_toa;

mod macros;
mod syntax_highlight;
mod schema;
mod stats;

pub use self::schema::{display_columns, schema_table};
pub use self::stats::stats_table;
pub use self::syntax_highlight::highlight;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
