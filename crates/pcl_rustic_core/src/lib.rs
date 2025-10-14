mod point;
mod point_cloud;

pub use point::Point;
pub use point_cloud::TablePointCloud;

pub fn hello_from_core() -> String {
    "Hello from pcl_rustic core!".to_string()
}
