//! Minimal PoC: skriv 4 små GeoTIFF-fliser (2x2) og flett dem til én GeoTIFF med terrio.
//!
//!   ┌──────┬──────┐
//!   │  0   │  1   │   hver flis: 64x64 celler à 1 m, EPSG:25833 (ETRS89 / UTM 33N)
//!   ├──────┼──────┤   verdi = flisindeks*1000 + rad, så skjøtene er lette å se
//!   │  2   │  3   │
//!   └──────┴──────┘

use terrio::ops::mosaic::{MosaicOptions, mosaic_to_path};
use terrio::raster::{CrsInfo, RasterData};
use terrio::{DataType, RasterConfig, RasterDataset, RasterWriteOptions};

const N: usize = 64; // celler per flis-side
const CELL: f64 = 1.0; // meter
const X0: f64 = 500_000.0; // vestkant for hele mosaikken
const Y0: f64 = 7_000_000.0; // sørkant for hele mosaikken
const NODATA: f64 = -9999.0;

fn main() -> terrio::Result<()> {
    std::fs::create_dir_all("out")?;
    let mut paths = Vec::new();

    for (i, (col, row)) in [(0, 1), (1, 1), (0, 0), (1, 0)].into_iter().enumerate() {
        // row 1 = nordre rad, siden y_min er sørkant
        let cfg = RasterConfig {
            cols: N,
            rows: N,
            bands: 1,
            x_min: X0 + (col * N) as f64 * CELL,
            y_min: Y0 + (row * N) as f64 * CELL,
            cell_size: CELL,
            nodata: NODATA,
            data_type: DataType::F32,
            crs: CrsInfo::from_epsg(25833),
            ..Default::default()
        };
        // Rad-major, øverste rad først
        let data: Vec<f32> = (0..N * N).map(|k| (i * 1000 + k / N) as f32).collect();
        let raster = terrio::raster_from_data(cfg, RasterData::F32(data))?;
        let ds = RasterDataset::new(raster, true);

        let path = format!("out/tile_{i}.tif");
        terrio::write_raster_dataset(&ds, &path, &RasterWriteOptions::default())?;
        println!("skrev {path}");
        paths.push(path);
    }

    // Flett: samme CRS og oppløsning, hele piksel-offset → terrio kopierer
    // strippevis uten å lese alt inn i minnet.
    let write_opts = RasterWriteOptions {
        compression: Some("DEFLATE".into()),
        ..Default::default()
    };
    mosaic_to_path(&paths, &MosaicOptions::default(), "out/merged.tif", &write_opts)?;
    println!("skrev out/merged.tif");

    // Rask kontroll
    let m = terrio::open_raster("out/merged.tif")?;
    let r = &m.raster;
    println!(
        "merged: {}x{} celler, x_min={} y_min={} celle={} epsg={:?}",
        r.cols, r.rows, r.x_min, r.y_min, r.cell_size_x, r.crs.epsg
    );
    Ok(())
}
