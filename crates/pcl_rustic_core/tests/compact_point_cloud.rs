use pcl_rustic_core::CompactPointCloud;
use pcl_rustic_core::Point;
use pcl_rustic_core::PointCloud;

#[test]
fn compact_add_and_attributes() {
    let mut pc = CompactPointCloud::new();
    assert_eq!(pc.num_points(), 0);

    let p1 = Point::new_2d(1.0, 2.0);
    pc.add_point(p1);
    assert_eq!(pc.num_points(), 1);
    assert!(!pc.is_3d());
    assert!(!pc.has_color());
    assert!(!pc.has_intensity());

    let p2 = Point::new_3d(3.0, 4.0, 5.0).with_rgba(10, 20, 30, 40).with_intensity(0.5);
    pc.add_point(p2);
    assert_eq!(pc.num_points(), 2);
    // after adding a 3D point the cloud reports is_3d
    assert!(pc.is_3d());
    assert!(pc.has_color());
    assert!(pc.has_intensity());
}
