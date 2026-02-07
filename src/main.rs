#![warn(missing_debug_implementations)]

use std::{env, path::Path, fs::File, error::Error};
use tabled::{Tabled, Table, settings::Style};
use tabled::assert::assert_table;

fn read_csv<P: AsRef<Path>>(filename: P, max_lines: Option<usize>) -> Result<(), Box<dyn Error>>{
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?.clone();

    let mut rows = Vec::new();
    for (i, result) in rdr.records().enumerate(){
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        // println!("{:?}", record);

        rows.push(row);
    }
    

    let mut builder = tabled::builder::Builder::default();
    
    builder.push_record(headers.iter());
    for (i, row) in rows.iter().enumerate() {
        if let Some(limit) = max_lines {
            if i >= limit {
                break;
            }
        }
        builder.push_record(row);
    }
    
    let table = builder.build()
        .with(Style::modern())
        .to_string();
    
    println!("{}", table);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let filename = &args[1];
    let n_lines: usize = args[2].parse()?;

    read_csv(filename,Some(n_lines));

    Ok(())
}
