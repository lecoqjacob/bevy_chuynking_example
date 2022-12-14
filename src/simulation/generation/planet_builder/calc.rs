use super::*;

pub fn planet_neighbors_four_way(idx: usize) -> [(Direction, usize); 4] {
    let mut result = [
        (Direction::North, 0),
        (Direction::South, 0),
        (Direction::East, 0),
        (Direction::West, 0),
    ];

    let (px, py) = idx_planet(idx);

    // West
    if px > 0 {
        result[3].1 = planet_idx(px - 1, py);
    } else {
        result[3].1 = planet_idx(WORLD_WIDTH - 1, py);
    }

    // East
    if px < WORLD_WIDTH - 1 {
        result[2].1 = planet_idx(px + 1, py);
    } else {
        result[2].1 = planet_idx(0, py);
    }

    // North
    let distance_from_middle = (WORLD_WIDTH as isize / 2) - px as isize;
    if py > 0 {
        result[0].1 = planet_idx(px, py);
    } else {
        result[0].1 = planet_idx((px as isize + distance_from_middle) as usize, py);
    }

    // South
    if py < WORLD_HEIGHT - 1 {
        result[1].1 = planet_idx(px, py);
    } else {
        result[1].1 = planet_idx((px as isize + distance_from_middle) as usize, py);
    }

    result
}

pub fn noise_lon(world_x: usize, region_x: usize) -> f32 {
    let x_extent = world_x as f32 / WORLD_WIDTH as f32;
    let sub_x = region_x as f32 / REGION_WIDTH as f32;
    (x_extent * 360.0) + sub_x - 180.0
}

pub fn noise_lat(world_y: usize, region_y: usize) -> f32 {
    let y_extent = world_y as f32 / WORLD_HEIGHT as f32;
    let sub_y = region_y as f32 / REGION_HEIGHT as f32;
    (y_extent * 180.0) + sub_y - 90.0
}

pub fn average_temperature_by_latitude(lat: Degrees) -> f32 {
    // Source: https://davidwaltham.com/global-warming-model/
    const AVERAGE_EQUATORIAL_C: f32 = 30.0;
    const A: f32 = 35.0; // Based on current data
    let lat_rad: Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    AVERAGE_EQUATORIAL_C - (A * lat_sin_squared)
}

pub fn average_precipitation_mm_by_latitude(lat: Degrees) -> f32 {
    // Mangled from https://i.stack.imgur.com/YBgot.png
    const PEAK: f32 = 8000.0;
    let fudge = if (lat.0 > -50.0 && lat.0 < -5.0) || (lat.0 < 50.0 && lat.0 > 5.0) {
        400.0
    } else {
        0.0
    };
    let lat_rad: Radians = lat.into();
    let lat_sin_squared = lat_rad.0.sin() * lat_rad.0.sin();
    PEAK - (lat_sin_squared * PEAK) - fudge
}

pub fn temperature_decrease_by_altitude(altitude_meters: f32) -> f32 {
    (altitude_meters / 1000.0) * 6.5
}

pub fn atmospheric_pressure_by_elevation(altitude_meters: f32) -> f32 {
    (101_325.0 * (1.0 - 2.25577 * 0.00001 * altitude_meters).powf(5.25588)) / 1000.0
}

pub fn sphere_vertex<A: Into<Radians>>(altitude: f32, lat: A, lon: A) -> (f32, f32, f32) {
    let rlat = lat.into();
    let rlon = lon.into();
    let sinlat = f32::sin(rlat.0);
    let coslat = f32::cos(rlat.0);
    let sinlon = f32::sin(rlon.0);
    let coslon = f32::cos(rlon.0);
    (altitude * coslat * coslon, altitude * coslat * sinlon, altitude * sinlat)
}
