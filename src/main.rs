#![warn(missing_debug_implementations)]

use std::{env, path::Path, fs::File, error::Error};
use tabled::{settings::Style};
// use tabled::assert::assert_table;

fn calculate_column_mean(rows: &[Vec<String>], col_idx: usize) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;

    for row in rows {
        if let Some(value) = row.get(col_idx){
            match value.parse::<f64>(){
                Ok(value) => {
                    sum += value;
                    count += 1;
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }
    }
        
    let mean = if count > 0 {
        sum / count as f64
    } else {
        0.0
    };
    
    mean
}

fn calculate_column_median(rows: &[Vec<String>], col_idx: usize) -> Option<f64> {
    // Extract numeric values from the column
    let mut values: Vec<f64> = rows
        .iter()
        .filter_map(|row| {
            row.get(col_idx)?
                .parse::<f64>()
                .ok()
        })
        .collect();

    if values.is_empty() {
        return None;
    }

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = values.len();
    let mid = len / 2;

    // Handle even vs odd
    if len % 2 == 0 {
        Some((values[mid - 1] + values[mid]) / 2.0)
    } else {
        Some(values[mid])
    }
}


fn read_csv<P: AsRef<Path>>(filename: P, max_lines: Option<usize>) -> Result<(), Box<dyn Error>>{
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let headers = rdr.headers()?.clone();


    let mut data_rows = Vec::new();
    for result in rdr.records(){
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        data_rows.push(row);
    }
    let mut data_builder = tabled::builder::Builder::default();
    
    data_builder.push_record(headers.iter());
    let max = max_lines.unwrap_or(10);
    for (i, row) in data_rows.iter().enumerate() {
        if i >= max {
            break;
        }
        data_builder.push_record(row);
    }
    
    let data_table = data_builder.build()
        .with(Style::modern())
        .to_string();
    


    // Calculate stats

    // Add an empty column to headers
    let mut stats_headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    stats_headers.insert(0, String::new());

    // Mean calculation
    let means: Vec<f64> = (0..headers.len())
        .map(|col_idx|{
            calculate_column_mean(&data_rows, col_idx)
        })
        .collect();

    let medians: Vec<Option<f64>> = (0..headers.len())
        .map(|col_idx|{
            calculate_column_median(&data_rows, col_idx)
        })
        .collect();

    let mut stats_builder = tabled::builder::Builder::default();
    
    stats_builder.push_record(stats_headers.iter());

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

    let stats_table = stats_builder.build()
        .with(Style::modern())
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
