/// A 2D location, with x and y coordinates in meters.
#[derive(Debug, Clone)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Location {
        Location {
            x,
            y,
        }
    }
}

/// A pixel location, with a pixel index and a location.
#[derive(Debug, Clone)]
pub struct PixelLocation {
    index: u32,
    location: Location,
}

/// A spatial map allows us to assign locations to individual
/// pixels without mapping out every single one.
/// We define the locations of corners and linearly interpolate
/// the locations of the other pixels.
#[derive(Debug, Clone)]
pub struct SpatialMap {
    pixels: u32,
    corners: Vec<PixelLocation>,
}

impl SpatialMap {
    pub fn new(pixels: u32) -> SpatialMap {
        SpatialMap {
            pixels,
            corners: vec![],
        }
    }

    pub fn add_corner(&mut self, index: u32, location: Location) -> &mut Self {
        self.corners.push(PixelLocation {
            index,
            location,
        });

        self
    }

    /// Gets the location of a pixel by linearly interpolating
    /// between the corners.
    pub fn get_pixel_location(&self, index: u32) -> Location {
        let previous_corner = self.corners.iter().rev().find(|corner| corner.index <= index).unwrap();
        let next_corner = self.corners.iter().find(|corner| corner.index >= index).unwrap();

        let distance = next_corner.index - previous_corner.index;

        let ratio = (index - previous_corner.index) as f32 / distance as f32;

        Location {
            x: previous_corner.location.x + (next_corner.location.x - previous_corner.location.x) * ratio,
            y: previous_corner.location.y + (next_corner.location.y - previous_corner.location.y) * ratio,
        }
    }

    /// Gets the location of every pixel on the strip by linearly
    /// interpolating between the corners.
    pub fn get_individual_pixel_locations(&self) -> Vec<Location> {
        let mut locations = vec![];

        for i in 0..self.pixels {
            locations.push(self.get_pixel_location(i));
        }

        locations
    }
}