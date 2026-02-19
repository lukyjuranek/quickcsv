
pub fn column_rows(rows: &[Vec<String>]) -> usize {
    rows.len()   
}

pub fn column_nas(rows: &[Vec<String>], col_idx: usize) -> usize {
    let count = rows.iter()
        .filter(|row| row.get(col_idx).map_or(false, |v| v.parse::<f64>().is_err()))
        .count();

    count
}

pub fn column_std(rows: &[Vec<String>], col_idx: usize) -> f64 {
    let values: Vec<f64> = rows
        .iter()
        .filter_map(|row| {
            row.get(col_idx)?
                .parse::<f64>()
                .ok()
        })
        .collect();

    let n = values.len();
    if n < 2 {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / n as f64;

    let variance = values
        .iter()
        .map(|value| {
            let diff = mean - *value;
            diff * diff
        })
        .sum::<f64>() / (n as f64 - 1.0);

    variance.sqrt()
}

pub fn column_mean(rows: &[Vec<String>], col_idx: usize) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;

    for row in rows {
        if let Some(value) = row.get(col_idx){
            if let Ok(value) = value.parse::<f64>(){
                sum += value;
                count += 1;
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

pub fn column_quartiles(rows: &[Vec<String>], col_idx: usize) -> Option<(f64, f64, f64, f64, f64)> {
    let mut values: Vec<f64> = rows
        .iter()
        .filter_map(|row| {
            row.get(col_idx)?
                .parse::<f64>()
                .ok()
        })
        .collect();

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let len = values.len();
    let min = values[0];
    let max = values[len - 1];
    
    let median = if len % 2 == 0 {
        (values[len / 2 - 1] + values[len / 2]) / 2.0
    } else {
        values[len / 2]
    };
    
    let q1_pos = len / 4;
    let q3_pos = (3 * len) / 4;
    let q1 = values[q1_pos];
    let q3 = values[q3_pos];
    
    Some((min, q1, median, q3, max))
}
