use nalgebra::Scalar;
use std::collections::HashMap;

/// A 3D point with required coordinates and optional attributes
/// Generic over numeric type T that supports matrix operations
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T = f64>
where
    T: Scalar + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub attributes: HashMap<String, f64>,
}

impl<T> Point<T>
where
    T: Scalar + Copy,
{
    /// Create a new point with x, y, z coordinates
    pub fn new(x: T, y: T, z: T) -> Self {
        Point {
            x,
            y,
            z,
            attributes: HashMap::new(),
        }
    }

    /// Create a new point with x, y, z coordinates and attributes
    pub fn with_attributes(x: T, y: T, z: T, attributes: HashMap<String, f64>) -> Self {
        Point {
            x,
            y,
            z,
            attributes,
        }
    }

    /// Add or update an attribute
    pub fn set_attribute(&mut self, key: String, value: f64) {
        self.attributes.insert(key, value);
    }

    /// Get an attribute value
    pub fn get_attribute(&self, key: &str) -> Option<f64> {
        self.attributes.get(key).copied()
    }
}

impl<T> Default for Point<T>
where
    T: Scalar + Copy + Default,
{
    fn default() -> Self {
        Point::new(T::default(), T::default(), T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = Point::new(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
        assert!(point.attributes.is_empty());
    }

    #[test]
    fn test_point_default() {
        let point: Point<f64> = Point::default();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
        assert!(point.attributes.is_empty());
    }

    #[test]
    fn test_point_with_attributes() {
        let mut attrs = HashMap::new();
        attrs.insert("intensity".to_string(), 100.0);
        attrs.insert("red".to_string(), 255.0);

        let point = Point::with_attributes(1.0, 2.0, 3.0, attrs);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.get_attribute("intensity"), Some(100.0));
        assert_eq!(point.get_attribute("red"), Some(255.0));
    }

    #[test]
    fn test_point_set_get_attribute() {
        let mut point = Point::new(1.0, 2.0, 3.0);
        point.set_attribute("intensity".to_string(), 150.0);
        assert_eq!(point.get_attribute("intensity"), Some(150.0));
        assert_eq!(point.get_attribute("nonexistent"), None);
    }

    #[test]
    fn test_point_generic_types() {
        // Test with f32
        let point_f32 = Point::<f32>::new(1.0, 2.0, 3.0);
        assert_eq!(point_f32.x, 1.0f32);

        // Test with i32
        let point_i32 = Point::<i32>::new(1, 2, 3);
        assert_eq!(point_i32.x, 1);
    }
}
