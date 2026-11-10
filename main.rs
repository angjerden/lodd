use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Hva programmet skal gjøre. Variantene er plassholdere – juster etter behov.
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Action {
    /// Les CSV, sorter på geografisk indeks og skriv Parquet
    Sort,
    /// Beregn støy per celle og skriv GeoTIFF
    Noise,
}

#[derive(Debug, Parser)]
#[command(name = "lodd", version, about)]
struct Args {
    /// Handling som skal utføres
    #[arg(long, value_enum)]
    action: Action,

    /// Kolonnerekkefølge i input-filene, f.eks. "yxz" (lat, lon, dybde)
    #[arg(long, default_value = "yxz")]
    column_order: String,

    /// Antall header-rader som skal hoppes over
    #[arg(long, default_value_t = 0)]
    header_rows: usize,

    /// Skilletegn mellom verdier
    #[arg(long, default_value_t = ' ')]
    value_separator: char,

    /// Mappe det søkes etter input-filer i
    #[arg(long, default_value = ".")]
    input_dir: PathBuf,

    /// Regex-mønster for input-filnavn
    #[arg(long, default_value = r".*\.(csv|txt|xyz)$")]
    input_file_regex_pattern: String,

    /// Oppdragsgrense (shp, gpkg eller gml)
    #[arg(long)]
    boundary_file: Option<PathBuf>,

    /// Geografisk horisontalt CRS for input (standard: ETRS89)
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
            .map_or("(ingen)".into(), |p| p.display().to_string())
    );
    println!("geographical_horizontal_crs: {}", args.geographical_horizontal_crs);
}
