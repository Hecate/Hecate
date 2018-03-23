extern crate postgis;
extern crate protobuf;

use r2d2; 
use r2d2_postgres;

mod builder;
mod encoder;
pub mod geom_encoder;
pub mod geom;
pub mod grid;
pub mod screen;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub mod proto; // protoc --rust_out . proto.proto

#[cfg(test)]
mod builder_test;

#[cfg(test)]
mod geom_encoder_test;

pub use self::builder::{Tile, Layer, Feature, Value};
pub use self::encoder::{Decode, Encode};
pub use self::grid::{Grid};

#[derive(Debug, PartialEq)]
pub enum MVTError {
    NotFound,
    DB,
}

impl MVTError {
    pub fn to_string(&self) -> String {
        match *self {
            MVTError::NotFound => { String::from("Tile not found") },
            MVTError::DB => { String::from("Tile Database Error") }

        }
    }
}

pub fn db_get(conn: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, z: i64, x: i64, y: i64) -> Result<Option<proto::Tile>, MVTError> {
    let rows = match conn.query("
        SELECT tile
        FROM tiles
        WHERE
            z = $1
            AND x = $2
            AND y = $3
    ", &[&z, &x, &y]) {
        Ok(rows) => rows,
        Err(_) => { return Err(MVTError::DB); }
};

if rows.len() == 0 { return Ok(None); }

Ok(None)
    }

pub fn db_create(conn: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, z: &u8, x: &u32, y: &u32) -> Result<proto::Tile, MVTError> {
    let grid = Grid::web_mercator();
    let bbox = grid.tile_extent(*z, *x, *y);
    let mut tile = Tile::new(&bbox);

    let mut layer = Layer::new("data");

    let mut limit: Option<i64> = None;
    if *z < 10 { limit = Some(10) }
    else if *z < 14 { limit = Some(100) }

    let rows = conn.query("
        SELECT
            id,
            ST_Transform(geom::geometry,3857),
            GeometryType(geom)
        FROM geo
        WHERE
            geom && ST_Transform(ST_MakeEnvelope($1, $2, $3, $4, $5), 4326)
        ORDER BY ST_Area(geom)
        LIMIT $6
    ", &[&bbox.minx, &bbox.miny, &bbox.maxx, &bbox.maxy, &grid.srid, &limit]).unwrap();

    for row in rows.iter() {
        let id: i64 = row.get(0);
        let mut feature = Feature::new(row.get(1));
        feature.set_id(id as u64);
        feature.add_property("id", Value::String(id.to_string()));
        layer.add_feature(feature);
    }

    tile.add_layer(layer);

    Ok(tile.encode(&grid))
}


pub fn db_cache(conn: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, z: i64, x: i64, y: i64, tile: &proto::Tile) -> Result<(), MVTError> {
    match conn.query("
        INSERT INTO tiles (z, x, y, tiles)
            VALUES ($1, $2, $3, $4)
    ", &[&z, &x, &y, &tile.to_bytes().unwrap()]) {
        Err(_) => Err(MVTError::DB),
        _ => Ok(())
    }
}

pub fn get(conn: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, z: u8, x: u32, y: u32) -> Result<proto::Tile, MVTError> {
    match db_get(&conn, i64::from(z), i64::from(x), i64::from(y))? {
        Some(tile) => { return Ok(tile); }
        _ => ()
    };

    let tile = db_create(&conn, &z, &x, &y)?;

    Ok(tile)
}
