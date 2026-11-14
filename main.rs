use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Action {
    /// Read CSV, sort by geographic index and write Parquet
    Sort,
    /// Compute noise per cell and write GeoTIFF
    Noise,
}

#[derive(Debug, Parser)]
#[command(name = "lodd", version, about)]
struct Args {
    /// Action to perform
    #[arg(long, value_enum)]
    action: Action,

    /// Column order in the input files, e.g. "yxz" (lat, lon, depth)
    #[arg(long, default_value = "yxz")]
    column_order: String,

    /// Number of header rows to skip
    #[arg(long, default_value_t = 0)]
    header_rows: usize,

    /// Separator between values
    #[arg(long, default_value_t = ' ')]
    value_separator: char,

    /// Directory to search for input files in
    #[arg(long, default_value = ".")]
    input_dir: PathBuf,

    /// Regex pattern for input file names
    #[arg(long, default_value = r".*\.(csv|txt|xyz)$")]
    input_file_regex_pattern: String,

    /// Survey boundary (shp, gpkg or gml)
    #[arg(long)]
    boundary_file: Option<PathBuf>,

    /// Geographic horizontal CRS for input (default: ETRS89)
    #[arg(long, default_value = "EPSG:4937")]
    geographical_horizontal_crs: String,
}

fn main() {
    let args = Args::parse();

    println!("action:                      {:?}", args.action);
    println!("column_order:                {}", args.column_order);
    println!("header_rows:                 {}", args.header_rows);
    println!("value_separator:             {:?}", args.value_separator);
    println!("input_dir:                   {}", args.input_dir.display());
    println!("input_file_regex_pattern:    {}", args.input_file_regex_pattern);
    println!(
        "boundary_file:               {}",
        args.boundary_file
            .as_deref()
            .map_or("(none)".into(), |p| p.display().to_string())
    );
    println!("geographical_horizontal_crs: {}", args.geographical_horizontal_crs);
}
