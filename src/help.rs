pub fn print_help() {
    println!("quickcsv - A utility for quick CSV file analysis");
    println!();
    println!("USAGE:");
    println!("    quickcsv <filename> [lines]");
    println!();
    println!("ARGUMENTS:");
    println!("    <filename>    Path to the CSV file to analyze");
    println!("    [lines]       Optional number of data rows to display (default: 10)");
    println!();
    println!("DESCRIPTION:");
    println!("    Displays statistical summary and data preview for a CSV file.");
    println!("    Statistics include: count, NaN values, mean, median, and standard deviation.");
    println!();
    println!("EXAMPLES:");
    println!("    quickcsv data.csv           # Display first 10 rows");
    println!("    quickcsv data.csv 20        # Display first 20 rows");
}

