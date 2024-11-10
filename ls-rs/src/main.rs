use chrono::{DateTime, Datelike, Timelike, Utc};
use tabular::{Row, Table};

fn main() {
    let mut table = Table::new("{:>} {:<}/{:<}/{:<} {:>}:{:<}");
    for file in std::fs::read_dir(".").unwrap() {
        let file = file.unwrap();
        let metadata = std::fs::metadata(file.path()).unwrap();

        if let Ok(updated) = metadata.modified() {
            //
            let updated_time: DateTime<Utc> = updated.into();
            table.add_row(
                Row::new()
                    .with_cell(file.path().display())
                    .with_cell(updated_time.year())
                    .with_cell(updated_time.month())
                    .with_cell(updated_time.day())
                    .with_cell(updated_time.hour())
                    .with_cell(updated_time.minute()),
            );
        }
    }
    print!("{}", table);
}
