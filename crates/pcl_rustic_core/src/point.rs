use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use ndarray::arr2;

/// A point in a point cloud with required and optional attributes
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    // Required attributes
    pub x: f32,
    pub y: f32,

    // Optional attributes
    pub z: Option<f32>,
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub a: Option<u8>,
    pub intensity: Option<f32>,
    pub ring_id: Option<u16>,
    pub time_offset: Option<f64>,
}

impl Point {
    /// Create a new 2D point with only required attributes
    pub fn new_2d(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            z: None,
            r: None,
            g: None,
            b: None,
            a: None,
            intensity: None,
            ring_id: None,
            time_offset: None,
        }
    }

    /// Create a new 3D point with x, y, z coordinates
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z: Some(z),
            r: None,
            g: None,
            b: None,
            a: None,
            intensity: None,
            ring_id: None,
            time_offset: None,
        }
    }

    pub fn transform(&self, a2b: &arr2<f32; 4, 4>) -> Self {
        let point_vec = arr2(&[[self.x], [self.y], [self.z.unwrap_or(0.0)], [1.0]]);
        let transformed = a2b.dot(&point_vec);
        Self {
            x: transformed[[0, 0]],
            y: transformed[[1, 0]],
            z: Some(transformed[[2, 0]]),
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
            intensity: self.intensity,
            ring_id: self.ring_id,
            time_offset: self.time_offset,
        }
    }

    /// Create a new colored point with RGB values
    pub fn with_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.r = Some(r);
        self.g = Some(g);
        self.b = Some(b);
        self
    }

    /// Create a new colored point with RGBA values
    pub fn with_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.r = Some(r);
        self.g = Some(g);
        self.b = Some(b);
        self.a = Some(a);
        self
    }

    /// Add intensity value to the point
    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = Some(intensity);
        self
    }

    /// Add ring ID to the point (useful for LiDAR data)
    pub fn with_ring_id(mut self, ring_id: u16) -> Self {
        self.ring_id = Some(ring_id);
        self
    }

    /// Add time offset to the point
    pub fn with_time_offset(mut self, time_offset: f64) -> Self {
        self.time_offset = Some(time_offset);
        self
    }

    /// Calculate the Euclidean distance between two points in 2D
    pub fn distance_2d(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate the Euclidean distance between two points in 3D
    /// Returns None if either point doesn't have a z coordinate
    pub fn distance_3d(&self, other: &Point) -> Option<f32> {
        match (self.z, other.z) {
            (Some(z1), Some(z2)) => {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                let dz = z1 - z2;
                Some((dx * dx + dy * dy + dz * dz).sqrt())
            }
            _ => None,
        }
    }

    /// Get the RGB values as a tuple if available
    pub fn rgb(&self) -> Option<(u8, u8, u8)> {
        match (self.r, self.g, self.b) {
            (Some(r), Some(g), Some(b)) => Some((r, g, b)),
            _ => None,
        }
    }

    /// Get the RGBA values as a tuple if available
    pub fn rgba(&self) -> Option<(u8, u8, u8, u8)> {
        match (self.r, self.g, self.b, self.a) {
            (Some(r), Some(g), Some(b), Some(a)) => Some((r, g, b, a)),
            _ => None,
        }
    }

    /// Check if the point has 3D coordinates
    pub fn is_3d(&self) -> bool {
        self.z.is_some()
    }

    /// Check if the point has color information
    pub fn has_color(&self) -> bool {
        self.r.is_some() && self.g.is_some() && self.b.is_some()
    }


}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "Point({}, {}", self.x, self.y);

        if let Some(z) = self.z {
            let _ = write!(f, ", {}", z);
        }

        if let Some((r, g, b)) = self.rgb() {
            let _ = write!(f, ", RGB: [{}, {}, {}]", r, g, b);
        }

        if let Some(intensity) = self.intensity {
            let _ = write!(f, ", I: {}", intensity);
        }

        if let Some(ring_id) = self.ring_id {
            let _ = write!(f, ", Ring: {}", ring_id);
        }

        if let Some(time_offset) = self.time_offset {
            let _ = write!(f, ", Time: {}", time_offset);
        }

        write!(f, ")")
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 + z2),
                _ => self.z,
            },
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
            intensity: self.intensity,
            ring_id: self.ring_id,
            time_offset: self.time_offset,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: match (self.z, other.z) {
                (Some(z1), Some(z2)) => Some(z1 - z2),
                _ => self.z,
            },
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
            intensity: self.intensity,
            ring_id: self.ring_id,
            time_offset: self.time_offset,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z.map(|z| z * scalar),
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
            intensity: self.intensity.map(|i| i * scalar),
            ring_id: self.ring_id,
            time_offset: self.time_offset,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z.map(|z| z / scalar),
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
            intensity: self.intensity.map(|i| i / scalar),
            ring_id: self.ring_id,
            time_offset: self.time_offset,
        }
    }
}
