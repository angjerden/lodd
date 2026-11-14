//! Minimal PoC: write 4 small GeoTIFF tiles (2x2) and merge them into one GeoTIFF with terrio.
//!
//!   ┌──────┬──────┐
//!   │  0   │  1   │   each tile: 64x64 cells of 1 m, EPSG:25833 (ETRS89 / UTM 33N)
//!   ├──────┼──────┤   value = tile index*1000 + row, so the seams are easy to see
//!   │  2   │  3   │
//!   └──────┴──────┘

use terrio::ops::mosaic::{MosaicOptions, mosaic_to_path};
use terrio::raster::{CrsInfo, RasterData};
use terrio::{DataType, RasterConfig, RasterDataset, RasterWriteOptions};

const N: usize = 64; // cells per tile side
const CELL: f64 = 1.0; // metres
const X0: f64 = 500_000.0; // west edge of the whole mosaic
const Y0: f64 = 7_000_000.0; // south edge of the whole mosaic
const NODATA: f64 = -9999.0;

fn main() -> terrio::Result<()> {
    std::fs::create_dir_all("out")?;
    let mut paths = Vec::new();

    for (i, (col, row)) in [(0, 1), (1, 1), (0, 0), (1, 0)].into_iter().enumerate() {
        // row 1 = northern row, since y_min is the south edge
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
        // Row-major, top row first
        let data: Vec<f32> = (0..N * N).map(|k| (i * 1000 + k / N) as f32).collect();
        let raster = terrio::raster_from_data(cfg, RasterData::F32(data))?;
        let ds = RasterDataset::new(raster, true);

        let path = format!("out/tile_{i}.tif");
        terrio::write_raster_dataset(&ds, &path, &RasterWriteOptions::default())?;
        println!("wrote {path}");
        paths.push(path);
    }

    // Merge: same CRS and resolution, whole-pixel offsets → terrio copies
    // strip by strip without reading everything into memory.
    let write_opts = RasterWriteOptions {
        compression: Some("DEFLATE".into()),
        ..Default::default()
    };
    mosaic_to_path(&paths, &MosaicOptions::default(), "out/merged.tif", &write_opts)?;
    println!("wrote out/merged.tif");

    // Quick check
    let m = terrio::open_raster("out/merged.tif")?;
    let r = &m.raster;
    println!(
        "merged: {}x{} cells, x_min={} y_min={} cell={} epsg={:?}",
        r.cols, r.rows, r.x_min, r.y_min, r.cell_size_x, r.crs.epsg
    );
    Ok(())
}
