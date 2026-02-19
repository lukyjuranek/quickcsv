#![warn(missing_debug_implementations)]

use std::{env, path::Path, fs::File, error::Error};
use tabled::{
    settings::{Style, 
        {style::LineText},
        merge::Merge, Alignment,formatting::Justification, object::Rows, themes::Colorization, Color}
};
mod utils;
use utils::{column_rows, column_nas, column_mean, column_median, column_std};

fn read_csv<P: AsRef<Path>>(filename: P, max_lines: Option<usize>) -> Result<(), Box<dyn Error>>{
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?.clone();

    // Add an empty column to headers
    let mut data_headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    data_headers.insert(0, String::new());


    let mut data_rows = Vec::new();
    for result in rdr.records(){
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        data_rows.push(row);
    }
    let mut data_builder = tabled::builder::Builder::default();
    
    data_builder.push_record(data_headers.iter());
    let max = max_lines.unwrap_or(10);
    for (i, row) in data_rows.iter().enumerate() {
        if i >= max {
            break;
        }
        // Prepend index to each row
        let mut indexed_row = vec![(i).to_string()];
        indexed_row.extend(row.iter().cloned());
        data_builder.push_record(indexed_row);
    }
    
    let data_table = data_builder.build()
        .with(Style::modern())
        .modify(Rows::first(), Color::BG_BLUE | Color::FG_BLACK)
        .with(LineText::new("Data", Rows::first()).offset(2))
        .to_string();
    


    // Calculate stats

    // Add an empty column to headers
    let mut stats_headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    stats_headers.insert(0, String::new());

    let rows: Vec<usize> = (0..headers.len())
        .map(|_|{
            column_rows(&data_rows)
        })
    .collect();
    
    let nas: Vec<usize> = (0..headers.len())
        .map(|col_idx|{
            column_nas(&data_rows, col_idx)
        })
        .collect();

    let means: Vec<f64> = (0..headers.len())
        .map(|col_idx|{
            column_mean(&data_rows, col_idx)
        })
        .collect();

    let medians: Vec<Option<f64>> = (0..headers.len())
        .map(|col_idx|{
            column_median(&data_rows, col_idx)
        })
        .collect();

    let std: Vec<f64> = (0..headers.len())
        .map(|col_idx|{
            column_std(&data_rows, col_idx)
        })
        .collect();

    let mut stats_builder = tabled::builder::Builder::default();
    
    stats_builder.push_record(stats_headers.iter());



    // Push count
    stats_builder.push_record({
        let mut row = vec!["count".to_string()];
        row.extend(rows.iter().map(|v| format!("{:.3}",v)));
        row
    });

    // Push nas 
    stats_builder.push_record({
        let mut row = vec!["NaN".to_string()];
        row.extend(nas.iter().map(|v| {
            if *v > 0 {
                format!("{:.3}", v)
            } else {
                String::new()
            }
        }));
        row
    });

    // Push means
    stats_builder.push_record({
        let mut row = vec!["mean".to_string()];
        row.extend(means.iter().map(|v| format!("{:.3}",v)));
        row
    });

    // Push medians
    stats_builder.push_record({
        let mut row = vec!["median".to_string()];
        row.extend(medians.iter().map(|v| match v{
            Some(val) => format!("{:.3}",val),
            None => "-".to_string(),
        }));
        row
    });

    // Push standard deviations 
    stats_builder.push_record({
        let mut row = vec!["std".to_string()];
        row.extend(std.iter().map(|v| format!("{:.3}",v)));
        row
    });

    let stats_table = stats_builder.build()
        .with(Style::modern())
        .with(Merge::horizontal())
        .modify(Rows::first(), Color::BG_BLUE | Color::FG_BLACK)
        //.with(Alignment::center())
        .with(LineText::new("Stats", Rows::first()).offset(2))
        .to_string();
    
    println!("{}", stats_table);
    println!("{}", data_table);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let filename = &args[1];
    let n_lines: Option<usize> = args.get(2)
        .map(|s| s.parse())
        .transpose()?;

    let _ = read_csv(filename,n_lines);

    Ok(())
}








