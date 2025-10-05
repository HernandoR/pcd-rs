// Re-export modules
mod point;
mod point_cloud;
mod compact_point_cloud;

pub use point::Point;
pub use point_cloud::PointCloud;
pub use compact_point_cloud::CompactPointCloud;

pub fn hello_from_core() -> String {
    "Hello from pcl_rustic core!".to_string()
}
