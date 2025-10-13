use polars::prelude::*;
use std::collections::HashMap;

pub fn hello_from_core() -> String {
    "Hello from pcl_rustic core!".to_string()
}

/// A 3D point with required coordinates and optional attributes
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub attributes: HashMap<String, f64>,
}

impl Point {
    /// Create a new point with x, y, z coordinates
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            x,
            y,
            z,
            attributes: HashMap::new(),
        }
    }

    /// Create a new point with x, y, z coordinates and attributes
    pub fn with_attributes(x: f64, y: f64, z: f64, attributes: HashMap<String, f64>) -> Self {
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

impl Default for Point {
    fn default() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }
}

/// A table-based point cloud using polars DataFrame
/// Each attribute is stored as a column
#[derive(Debug, Clone)]
pub struct TablePointCloud {
    data: DataFrame,
}

impl TablePointCloud {
    /// Create a new empty TablePointCloud
    pub fn new() -> Result<Self, PolarsError> {
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let z: Vec<f64> = vec![];

        let df = DataFrame::new(vec![
            Series::new("x".into(), x).into(),
            Series::new("y".into(), y).into(),
            Series::new("z".into(), z).into(),
        ])?;

        Ok(TablePointCloud { data: df })
    }

    /// Create a TablePointCloud from vectors of coordinates
    pub fn from_xyz(x: Vec<f64>, y: Vec<f64>, z: Vec<f64>) -> Result<Self, PolarsError> {
        if x.len() != y.len() || y.len() != z.len() {
            return Err(PolarsError::ShapeMismatch(
                "x, y, z vectors must have the same length".into(),
            ));
        }

        let df = DataFrame::new(vec![
            Series::new("x".into(), x).into(),
            Series::new("y".into(), y).into(),
            Series::new("z".into(), z).into(),
        ])?;

        Ok(TablePointCloud { data: df })
    }

    /// Create a TablePointCloud from a vector of Points
    pub fn from_points(points: Vec<Point>) -> Result<Self, PolarsError> {
        if points.is_empty() {
            return Self::new();
        }

        let x: Vec<f64> = points.iter().map(|p| p.x).collect();
        let y: Vec<f64> = points.iter().map(|p| p.y).collect();
        let z: Vec<f64> = points.iter().map(|p| p.z).collect();

        let mut series = vec![
            Series::new("x".into(), x).into(),
            Series::new("y".into(), y).into(),
            Series::new("z".into(), z).into(),
        ];

        // Collect all unique attribute keys
        let mut all_keys: Vec<String> = points
            .iter()
            .flat_map(|p| p.attributes.keys().cloned())
            .collect();
        all_keys.sort();
        all_keys.dedup();

        // Add columns for each attribute
        for key in all_keys {
            let values: Vec<f64> = points
                .iter()
                .map(|p| p.attributes.get(&key).copied().unwrap_or(f64::NAN))
                .collect();
            series.push(Series::new(key.as_str().into(), values).into());
        }

        let df = DataFrame::new(series)?;
        Ok(TablePointCloud { data: df })
    }

    /// Get the number of points in the cloud
    pub fn len(&self) -> usize {
        self.data.height()
    }

    /// Check if the point cloud is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add a column (attribute) to the point cloud
    pub fn add_attribute(&mut self, name: &str, values: Vec<f64>) -> Result<(), PolarsError> {
        if values.len() != self.len() {
            return Err(PolarsError::ShapeMismatch(
                format!(
                    "Attribute length {} does not match point cloud length {}",
                    values.len(),
                    self.len()
                )
                .into(),
            ));
        }

        let series = Series::new(name.into(), values);
        self.data.with_column(series)?;
        Ok(())
    }

    /// Get a reference to the underlying DataFrame
    pub fn data(&self) -> &DataFrame {
        &self.data
    }

    /// Get x coordinates as a vector
    pub fn x(&self) -> Result<Vec<f64>, PolarsError> {
        let series = self.data.column("x")?;
        Ok(series
            .f64()?
            .to_vec()
            .into_iter()
            .map(|v| v.unwrap_or(0.0))
            .collect())
    }

    /// Get y coordinates as a vector
    pub fn y(&self) -> Result<Vec<f64>, PolarsError> {
        let series = self.data.column("y")?;
        Ok(series
            .f64()?
            .to_vec()
            .into_iter()
            .map(|v| v.unwrap_or(0.0))
            .collect())
    }

    /// Get z coordinates as a vector
    pub fn z(&self) -> Result<Vec<f64>, PolarsError> {
        let series = self.data.column("z")?;
        Ok(series
            .f64()?
            .to_vec()
            .into_iter()
            .map(|v| v.unwrap_or(0.0))
            .collect())
    }

    /// Get a point at a specific index
    pub fn get_point(&self, index: usize) -> Result<Point, PolarsError> {
        if index >= self.len() {
            return Err(PolarsError::OutOfBounds(
                format!("Index {} out of bounds for length {}", index, self.len()).into(),
            ));
        }

        let x = self.data.column("x")?.f64()?.get(index).unwrap_or(0.0);
        let y = self.data.column("y")?.f64()?.get(index).unwrap_or(0.0);
        let z = self.data.column("z")?.f64()?.get(index).unwrap_or(0.0);

        let mut attributes = HashMap::new();
        for col_name in self.data.get_column_names() {
            if col_name != "x" && col_name != "y" && col_name != "z" {
                if let Ok(series) = self.data.column(col_name) {
                    if let Ok(f64_series) = series.f64() {
                        if let Some(value) = f64_series.get(index) {
                            if !value.is_nan() {
                                attributes.insert(col_name.to_string(), value);
                            }
                        }
                    }
                }
            }
        }

        Ok(Point {
            x,
            y,
            z,
            attributes,
        })
    }

    /// Convert to a vector of Points
    pub fn to_points(&self) -> Result<Vec<Point>, PolarsError> {
        let mut points = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            points.push(self.get_point(i)?);
        }
        Ok(points)
    }
}

impl Default for TablePointCloud {
    fn default() -> Self {
        Self::new().expect("Failed to create default TablePointCloud")
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
        let point = Point::default();
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
    fn test_table_point_cloud_new() {
        let cloud = TablePointCloud::new().unwrap();
        assert_eq!(cloud.len(), 0);
        assert!(cloud.is_empty());
    }

    #[test]
    fn test_table_point_cloud_from_xyz() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![4.0, 5.0, 6.0];
        let z = vec![7.0, 8.0, 9.0];

        let cloud = TablePointCloud::from_xyz(x, y, z).unwrap();
        assert_eq!(cloud.len(), 3);
        assert!(!cloud.is_empty());

        let x_coords = cloud.x().unwrap();
        assert_eq!(x_coords, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_table_point_cloud_from_points() {
        let mut points = vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)];

        points[0].set_attribute("intensity".to_string(), 100.0);
        points[1].set_attribute("intensity".to_string(), 200.0);

        let cloud = TablePointCloud::from_points(points).unwrap();
        assert_eq!(cloud.len(), 2);

        let point0 = cloud.get_point(0).unwrap();
        assert_eq!(point0.x, 1.0);
        assert_eq!(point0.y, 2.0);
        assert_eq!(point0.z, 3.0);
        assert_eq!(point0.get_attribute("intensity"), Some(100.0));
    }

    #[test]
    fn test_table_point_cloud_add_attribute() {
        let x = vec![1.0, 2.0];
        let y = vec![3.0, 4.0];
        let z = vec![5.0, 6.0];

        let mut cloud = TablePointCloud::from_xyz(x, y, z).unwrap();
        cloud
            .add_attribute("intensity", vec![100.0, 200.0])
            .unwrap();

        let point0 = cloud.get_point(0).unwrap();
        assert_eq!(point0.get_attribute("intensity"), Some(100.0));
    }

    #[test]
    fn test_table_point_cloud_to_points() {
        let points = vec![Point::new(1.0, 2.0, 3.0), Point::new(4.0, 5.0, 6.0)];

        let cloud = TablePointCloud::from_points(points.clone()).unwrap();
        let recovered_points = cloud.to_points().unwrap();

        assert_eq!(recovered_points.len(), 2);
        assert_eq!(recovered_points[0].x, 1.0);
        assert_eq!(recovered_points[1].x, 4.0);
    }

    #[test]
    fn test_point_cloud_mismatched_lengths() {
        let x = vec![1.0, 2.0];
        let y = vec![3.0, 4.0, 5.0]; // Different length
        let z = vec![6.0, 7.0];

        let result = TablePointCloud::from_xyz(x, y, z);
        assert!(result.is_err());
    }
}
