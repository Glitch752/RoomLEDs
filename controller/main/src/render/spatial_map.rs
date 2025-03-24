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

    pub fn from_inches(x: f32, y: f32) -> Location {
        Location {
            x: x * 0.0254,
            y: y * 0.0254,
        }
    }

    pub fn lerp(&self, other: &Location, ratio: f32) -> Location {
        Location {
            x: self.x + (other.x - self.x) * ratio,
            y: self.y + (other.y - self.y) * ratio,
        }
    }
}

/// A span of pixel locations between two corners.
/// The start index is inclusive, while the end index is exclusive.
#[derive(Debug, Clone)]
pub struct PixelSpan {
    start_index: u32,
    end_index: u32,
    start_location: Location,
    end_location: Location
}

impl PixelSpan {
    pub fn new(start_index: u32, end_index: u32, start_location: Location, end_location: Location) -> PixelSpan {
        PixelSpan {
            start_index,
            end_index,
            start_location,
            end_location,
        }
    }

    pub fn contains(&self, index: u32) -> bool {
        index >= self.start_index && index < self.end_index
    }

    pub fn get_location(&self, index: u32) -> Location {
        let distance = self.end_index - self.start_index;

        let ratio = (index - self.start_index) as f32 / distance as f32;
        self.start_location.lerp(&self.end_location, ratio)
    }
}

/// A spatial map allows us to assign locations to individual
/// pixels without mapping out every single one.
/// We define the locations of spans and linearly interpolate
/// the locations of the other pixels.
#[derive(Debug, Clone)]
pub struct SpatialMap {
    pixels: u32,
    spans: Vec<PixelSpan>,
}

impl SpatialMap {
    pub fn new(pixels: u32) -> SpatialMap {
        SpatialMap {
            pixels,
            spans: vec![],
        }
    }

    pub fn add_span(&mut self, start_index: i32, end_index: i32, start_location: Location, end_location: Location) -> &mut Self {
        // If the start index is negative, split the span into two (one at the end and one at the start)
        if end_index < 0 {
            panic!("End index cannot be negative");
        }

        if start_index < 0 {
            let ratio = (-start_index) as f32 / (end_index - start_index) as f32;
            let split_location = start_location.lerp(&end_location, ratio);
            
            self.spans.push(PixelSpan::new(((self.pixels as i32) + start_index) as u32, self.pixels, start_location, split_location.clone()));
            self.spans.push(PixelSpan::new(0, end_index as u32, split_location, end_location));
            return self;
        }
        
        self.spans.push(PixelSpan::new(start_index as u32, end_index as u32, start_location, end_location));
        self
    }

    /// Gets the location of a pixel by linearly interpolating
    pub fn get_pixel_location(&self, index: u32) -> Location {
        for span in &self.spans {
            if span.contains(index) {
                return span.get_location(index);
            }
        }

        panic!("Pixel index {} not found in any span", index);
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